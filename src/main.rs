//! A simple CLI for the nice_rust library.

extern crate nice_rust;

extern crate clap;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(
        value_enum,
        default_value = "detailed",
        help = "the checkout mode to use"
    )]
    mode: nice_rust::Mode,

    #[arg(
        long,
        default_value = "https://nicenumbers.net/api",
        help = "the base API URL to connect to"
    )]
    api_base: String,

    #[arg(
        short,
        long,
        default_value = "anonymous",
        help = "the username to send alongside your contribution"
    )]
    username: String,

    #[arg(short, long, help = "suppress some output")]
    quiet: bool,

    #[arg(short, long, help = "show additional output")]
    verbose: bool,

    #[arg(long, help = "run an offline benchmark")]
    benchmark: bool,

    #[arg(short, long, help = "request a range in a specific base")]
    base: Option<u32>,

    #[arg(short = 'r', long, help = "request a differently-sized range")]
    max_range: Option<u128>,

    #[arg(long, help = "request a specific field by id")]
    field: Option<u128>,
}

fn main() {
    // parse args from command line
    let cli = Cli::parse();
    nice_rust::run(
        cli.mode,
        cli.api_base,
        cli.username,
        cli.quiet,
        cli.verbose,
        cli.benchmark,
        cli.base,
        cli.max_range,
        cli.field,
    );
}
