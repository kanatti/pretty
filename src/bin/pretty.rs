use clap::Parser;

use pretty::args::Args;

fn main() {
    let args = Args::parse();

    pretty::run(args);
}
