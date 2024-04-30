// region:		--- mod
mod config;
mod error;
mod file;
mod parser;
mod prelude;
mod statistics;
// endregion:	--- mod

// region:		--- use
pub use self::error::{Error, Result};
use config::Config;
pub use prelude::*;
// endregion:	--- use

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let args = Args::new(args);

    let stats = match args {
        Args::Empty => {
            println!("{}", default_help_text());
            return Ok(());
        }
        Args::Help => {
            println!("{}", default_help_text());
            return Ok(());
        }
        Args::Xml => {
            let config = Config::new()?;
            let mut json_file = Box::new(JsonFile::new(config.file_loc)?);
            json_file.set_content()?;
            json_file.write_as_xml(config.cur_dir.join("output"), ".xml")?
        }
        Args::Plain => {
            let config = Config::new()?;
            let mut json_file = Box::new(JsonFile::new(config.file_loc)?);
            json_file.set_content()?;
            json_file.write_as_plain(config.cur_dir.join("output"), ".xml")?
        }
    };

    let stat_out = stats.wrap_get();
    println!("\nStats output: {}", stat_out);

    Ok(())
}
