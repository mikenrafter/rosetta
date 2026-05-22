use clap::Parser;
use rosetta::cli::Cli;
use rosetta::detect;
use rosetta::input;
use rosetta::output;
use std::io::{self, IsTerminal};

fn main() {
    let cli = Cli::parse();
    let max_bytes = input::resolve_max_bytes(cli.max_input_bytes);

    let input = match get_input(&cli, max_bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    let input = input.trim();

    if input.is_empty() {
        eprintln!("Usage: rosetta <data>");
        eprintln!("       echo <data> | rosetta");
        eprintln!();
        eprintln!("Pipe in or pass any opaque data — timestamps, JWTs, base64,");
        eprintln!("UUIDs, cron expressions, colors, IPs, and more.");
        std::process::exit(1);
    }

    let results = detect::run_all(input);

    if results.is_empty() {
        output::print_no_match(input);
    } else {
        output::print_results(&results);
    }
}

fn get_input(cli: &Cli, max_bytes: usize) -> io::Result<String> {
    if !cli.data.is_empty() {
        return Ok(cli.data.join(" "));
    }
    if io::stdin().is_terminal() {
        return Ok(String::new());
    }
    input::read_bounded(io::stdin().lock(), max_bytes)
}
