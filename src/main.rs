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

    let start = std::time::Instant::now();

    match args {
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
            let json_file = JsonFile::new(config.file_loc)?
                .set_content()?
                .write_as_xml(config.cur_dir.join("output"), ".xml")?
                .wrap_get();
            println!("{}", json_file);
        }
        Args::Plain => {
            let config = Config::new()?;
            let json_file = JsonFile::new(config.file_loc)?
                .set_content()?
                .write_as_plain(config.cur_dir.join("output"), ".xml")?
                .wrap_get();
            println!("{}", json_file);
        }
    };
    let end = start.elapsed();

    println!("Runtime: {:?}", end);

    // let stat_out = stats.wrap_get();
    // println!("\nStats output: {}", stat_out);

    Ok(())
}
