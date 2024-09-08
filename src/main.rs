mod head_cli;

use clap::Parser;
use head_cli::HeadCli;

fn main() {
    let cmd = HeadCli::parse();
    if let Err(e) = cmd.run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
