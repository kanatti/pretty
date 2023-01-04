use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub file_name: String,

    /// Comma seperated list of fields to flatten
    #[arg(
        short,
        long,
        value_name = "FIELDS",
        use_value_delimiter = true,
        value_delimiter(',')
    )]
    pub flatten: Vec<String>,
}
