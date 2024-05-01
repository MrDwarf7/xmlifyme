#[derive(Clone, Debug)]
pub enum Args {
    Xml,
    Plain,
    Help,
    Empty,
}

impl Args {
    pub fn new(mut args: Vec<String>) -> Self {
        args.remove(0);

        if args.is_empty() {
            return Args::Empty;
        }

        // Match out the - or --
        let args = args
            .iter()
            .map(|arg| {
                if arg.starts_with('-') {
                    arg.trim_start_matches('-')
                } else {
                    match arg.starts_with("--") {
                        true => arg.trim_start_matches("--"),
                        false => arg,
                    }
                }
            })
            .collect::<String>();

        match args.as_str() {
            "xml" => Args::Xml,
            "x" => Args::Xml,
            "plain" => Args::Plain,
            "p" => Args::Plain,
            "help" => Args::Help,
            "h" => Args::Help,
            _ => Args::Empty,
        }
    }
}

pub fn default_help_text() -> &'static str {
    "\nUsage: xmlifyme [xml|plain|help]
info:
    input file should be in '.\\data\\' folder,
    output will be in '.\\output\\' folder\n
    xml: Output as xml
    plain: Output as plain text in a .xml file
    help: Display this help text"
}
