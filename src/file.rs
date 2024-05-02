// region:		--- use
use crate::{prelude::*, statistics::Statistics, Result};
use quick_xml::events::BytesStart;
use std::{
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};
// endregion:	--- use

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessEntry {
    pub name: String,
    pub processxml: String,
}

impl From<String> for ProcessEntry {
    fn from(content: String) -> Self {
        let content: ProcessEntry = serde_json::from_str(&content).unwrap();

        ProcessEntry {
            name: content.name,
            processxml: content.processxml,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonFile {
    pub location: PathBuf,
    pub content: Vec<ProcessEntry>,
}

// region:		--- Setup
impl JsonFile {
    pub fn new<P: AsRef<Path>>(location: P) -> Result<Self> {
        let location = location.as_ref().to_path_buf();

        Ok(JsonFile {
            location,
            content: Vec::new(),
        })
    }

    pub fn set_content(&mut self) -> Result<Self> {
        let location = &self.location;
        let file = std::fs::File::open(location)?;
        let mut read = BufReader::new(file);

        let mut c = Vec::new();
        read.read_to_end(&mut c)?;

        let pattern = r#"\\"#;
        let content = String::from_utf8_lossy(&c).replace(pattern, "");
        let content: Vec<ProcessEntry> = serde_json::from_str(&content).unwrap();

        let location = location.to_path_buf();

        Ok(JsonFile { location, content })
    }
}
// endregion:	--- Setup

impl JsonFile {
    pub fn write_as_xml<P: AsRef<Path>>(
        &self,
        output_dir: P,
        extension: &str,
    ) -> Result<Statistics> {
        if !output_dir.as_ref().exists() {
            std::fs::create_dir_all(output_dir.as_ref())?;
        }

        let mut stats = Statistics::default();

        for entry in self.content.iter() {
            // region:		--- setup on loop
            let file_name = Self::with_extension(entry.name.clone(), extension);
            let entry_contents = entry.processxml.clone();
            let out_file_name = output_dir.as_ref().join(&file_name);
            // endregion:	--- setup on loop

            // region:		--- Output
            let mut buffer = Vec::new();
            let mut writer = quick_xml::Writer::new_with_indent(&mut buffer, b' ', 2);
            let start = BytesStart::new("");
            let end = start.to_end().clone();

            let _ = writer.write_event(quick_xml::events::Event::Start(start.clone()));
            writer.write_serializable("process_data", &entry_contents)?;
            writer.write_event(quick_xml::events::Event::End(end.to_owned()))?;

            let out_file = std::fs::File::create(out_file_name)?;
            let mut buf_writer = BufWriter::new(out_file);
            buf_writer.write_all(&buffer)?;
            buf_writer.flush()?;
            // endregion:	--- Output

            // region:		--- Stats
            stats.wrap_input(
                entry_contents.chars().count(),
                entry_contents.len(),
                output_dir.as_ref().to_path_buf(),
                file_name.to_str().unwrap().to_string(),
            );
            // endregion:	--- Stats
        }
        Ok(stats)
    }

    pub fn write_as_plain<P: AsRef<Path>>(
        &self,
        output_dir: P,
        extension: &str,
    ) -> Result<Statistics> {
        if !output_dir.as_ref().exists() {
            std::fs::create_dir_all(output_dir.as_ref())?;
        }

        let mut stats = Statistics::default();

        self.content.iter().for_each(|entry| {
            // region:		--- setup on loop
            let file_name = Self::with_extension(entry.name.clone(), extension);
            let entry_contents = entry.processxml.clone();
            let out_file_name = output_dir.as_ref().join(&file_name);
            // endregion:	--- setup on loop

            // region:		--- Output
            let out_file = std::fs::File::create(out_file_name).unwrap();
            let buf_writer = BufWriter::new(out_file);
            serde_json::to_writer(buf_writer, &entry_contents).unwrap();
            // endregion:	--- Output

            // region:		--- Stats
            stats.wrap_input(
                entry_contents.chars().count(),
                entry_contents.len(),
                output_dir.as_ref().to_path_buf(),
                file_name.to_str().unwrap().to_string(),
            )
            // endregion:	--- Stats
        });
        Ok(stats)
    }
}

// region:		--- Private Helpers
impl JsonFile {
    fn with_extension<P: AsRef<Path>>(file_name: P, extension: &str) -> PathBuf {
        let mut str_lossy_file_name = file_name.as_ref().to_string_lossy();

        // Fix the forward slashes in the (process - which is also the name of the generated file)
        str_lossy_file_name.to_mut().push_str(extension);
        str_lossy_file_name = str_lossy_file_name.replace('/', "_").into();

        let new_file_name = str_lossy_file_name
            .as_ref()
            .parse::<String>()
            .unwrap()
            .parse::<PathBuf>()
            .unwrap();

        new_file_name
    }
}
// endregion:	--- Private Helpers

// region:		--- Boilerplate
impl From<&str> for JsonFile {
    fn from(content: &str) -> Self {
        let content: Vec<ProcessEntry> = serde_json::from_str(content).unwrap();

        JsonFile {
            location: PathBuf::new(),
            content,
        }
    }
}
// endregion:	--- Boilerplate
