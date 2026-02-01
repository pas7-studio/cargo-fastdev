// File: src/main.rs
use clap::{Parser, Subcommand};

use cargo_fastdev::{cmd_check, cmd_doctor, cmd_init, cmd_run, cmd_test, cmd_watch};

#[derive(Parser, Debug)]
#[command(name = "cargo-fastdev")]
#[command(version)]
#[command(about = "Fast Rust dev loop: doctor/init/watch + cargo wrappers")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Doctor {
        #[arg(short, long)]
        format: Option<String>,
    },
    Init {
        #[arg(long)]
        print: bool,
        #[arg(long)]
        write: bool,
        #[arg(long)]
        use_sccache: bool,
        #[arg(long)]
        use_mold: bool,
    },
    Watch {
        command: String,
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    Check {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    Test {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    Run {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Doctor { format } => cmd_doctor(format),
        Command::Init {
            print,
            write,
            use_sccache,
            use_mold,
        } => cmd_init(print, write, use_sccache, use_mold),
        Command::Watch { command, args } => cmd_watch(command, args),
        Command::Check { args } => cmd_check(args),
        Command::Test { args } => cmd_test(args),
        Command::Run { args } => cmd_run(args),
    }
}
