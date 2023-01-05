use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub file: String,

    /// Comma seperated list of fields to flatten
    #[arg(
        short,
        long,
        value_name = "FIELDS",
        use_value_delimiter = true,
        value_delimiter(',')
    )]
    pub flatten: Vec<String>,

    #[arg(short, long, value_enum, default_value_t=Color::Never)]
    pub color: Color,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone)]
pub enum Color {
    Never,
    Auto,
    Always,
}