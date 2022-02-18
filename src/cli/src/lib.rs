use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

pub fn init() {
    Cli::parse().run_command();
}

#[derive(Parser)]
#[clap(name = "zx")]
#[clap(about = "ZX is a simple programmimg language", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(arg_required_else_help = true)]
    Build {
        #[clap(required = true)]
        path: PathBuf,
        #[clap(short = 'o')]
        output: String
    },
    #[clap(arg_required_else_help = true)]
    Run {
        #[clap(required = true)]
        path: PathBuf
    }
}

impl Cli {
    fn run_command(self) {
        match self.command {
            SubCommand::Build { path, output } => {

            }
            SubCommand::Run { path } => {
                println!("{:?}", path)
            }
        }
    }
}