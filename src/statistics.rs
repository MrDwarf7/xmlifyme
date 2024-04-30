// region:		--- use
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};
// endregion:	--- use

// region:		--- struct
pub struct Statistics {
    pub char_count: usize,
    pub array_length: usize,
    pub output_dir: PathBuf,
    pub output_filename: String,
}

// region:		--- Setup
impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            char_count: 0,
            array_length: 0,
            output_dir: PathBuf::new(),
            output_filename: String::new(),
        }
    }
}
// endregion:	--- Setup

// region:		--- Wrapper
impl Statistics {
    pub fn wrap_set(&mut self, output_dir: PathBuf, output_filename: String) {
        self.set_output_dir(output_dir);
        self.set_output_filename(output_filename);
    }

    pub fn wrap_add(&mut self, char_count: usize, array_length: usize) {
        self.add_char_count(char_count);
        self.add_array_length(array_length);
    }

    pub fn wrap_get(&self) -> StatsOutput {
        let results = (
            self.get_char_count(),
            self.get_array_length(),
            self.get_output_dir(),
            self.get_output_filename(),
        );

        StatsOutput::new(
            results.0,
            results.1,
            results.2.to_path_buf(),
            results.3.to_string(),
        )
    }

    pub fn wrap_input(
        &mut self,
        char_count: usize,
        array_length: usize,
        output_dir: PathBuf,
        output_filename: String,
    ) {
        self.add_char_count(char_count);
        self.add_array_length(array_length);
        self.set_output_dir(output_dir);
        self.set_output_filename(output_filename);
    }
}

// endregion:	--- Wrapper

// region:		--- Setters
impl Statistics {
    pub fn add_char_count(&mut self, count: usize) {
        self.char_count += count;
    }

    pub fn add_array_length(&mut self, length: usize) {
        self.array_length += length;
    }

    pub fn set_output_dir(&mut self, output_dir: PathBuf) {
        self.output_dir = output_dir;
    }

    pub fn set_output_filename(&mut self, output_filename: String) {
        self.output_filename = output_filename;
    }
}
// endregion:	--- Setters

// region:		--- Getters
impl Statistics {
    pub fn get_char_count(&self) -> usize {
        self.char_count
    }

    pub fn get_array_length(&self) -> usize {
        self.array_length
    }

    pub fn get_output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    pub fn get_output_filename(&self) -> &str {
        &self.output_filename
    }
}
// endregion:	--- Getters

// region:		--- Boilerplate
impl Display for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Statistics: char_count: {}, array_length: {}",
            self.char_count, self.array_length
        )
    }
}

impl Debug for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n\tStatistics: char_count: {}\n, array_length: {}",
            self.char_count, self.array_length
        )
    }
}
// endregion:	--- Boilerplate
// endregion:	--- struct

// region:		--- struct
#[derive(Debug)]
pub struct StatsOutput {
    char_count: usize,
    array_length: usize,
    pub output_dir: PathBuf,
    pub output_filename: String,
}

impl StatsOutput {
    pub fn new(
        char_count: usize,
        array_length: usize,
        output_dir: PathBuf,
        output_filename: String,
    ) -> Self {
        StatsOutput {
            char_count,
            array_length,
            output_dir,
            output_filename,
        }
    }
}

// region:		--- Boilerplate
impl Display for StatsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "StatsOutput:\n char_count: {},\n array_length: {},\n output_dir: {},\n output_filename: {}\n",
            self.char_count, self.array_length, self.output_dir.to_str().unwrap(), self.output_filename
        )
    }
}

// endregion:	--- Boilerplate
// endregion:	--- struct
