//! A simple CLI for the nice_rust library.

extern crate nice_rust;

extern crate clap;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The checkout mode to use
    #[arg(value_enum, default_value = "detailed")]
    mode: nice_rust::Mode,

    /// The base API URL to connect to
    #[arg(long, default_value = "https://nicenumbers.net/api")]
    api_base: String,

    /// The username to send alongside your contribution
    #[arg(short, long, default_value = "anonymous")]
    username: String,

    /// Suppress some output
    #[arg(short, long)]
    quiet: bool,

    /// Show additional output
    #[arg(short, long)]
    verbose: bool,

    /// Run an offline benchmark [default: base 40, range 100000]
    #[arg(long)]
    benchmark: bool,

    /// Run indefinitely with the current settings
    #[arg(long)]
    repeat: bool,

    /// Enable experminetal support for inputs above 2^128
    /// This allows acces to bases above 97 but is slower
    #[arg(long, verbatim_doc_comment)]
    high_bases: bool,

    /// Request a range in a specific base
    /// The server may deny this request based on capacity
    #[arg(short, long, verbatim_doc_comment)]
    base: Option<u32>,

    /// Request a differently-sized range
    /// The server may deny this request based on capacity
    #[arg(short, long, verbatim_doc_comment)]
    range: Option<u32>,

    /// Request a specific field by ID
    /// The same username must be used to reclaim a field
    #[arg(long, verbatim_doc_comment)]
    field: Option<u32>,
}

fn main() {
    // parse args from command line
    let cli = Cli::parse();

    // loop if repeat is set
    while cli.repeat {
        nice_rust::run(
            cli.mode,
            cli.api_base.clone(),
            cli.username.clone(),
            cli.quiet,
            cli.verbose,
            cli.benchmark,
            cli.high_bases,
            cli.base,
            cli.range,
            cli.field,
        );
    }

    // or just run once
    nice_rust::run(
        cli.mode,
        cli.api_base,
        cli.username,
        cli.quiet,
        cli.verbose,
        cli.benchmark,
        cli.high_bases,
        cli.base,
        cli.range,
        cli.field,
    );
}
