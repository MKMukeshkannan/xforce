use clap::{Parser, Subcommand};
use generate::create_file;
use std::str;
use test::run_test;

mod generate;
mod test;

#[derive(Parser)]
struct Arguments {
    /// this is a top level command
    #[command(subcommand)]
    top_command: TopCommand,
}

#[derive(Subcommand)]
enum TopCommand {
    /// to create required files
    GENERATE {
        #[arg(short, long)]
        input: Option<String>,

        #[arg(short, long)]
        req_output: Option<String>,
    },
    /// To Test Output and Required output
    TEST {
        cses: Option<String>,
    },
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    match &args.top_command {
        TopCommand::GENERATE { input, req_output } => {
            create_file(input.clone(), req_output.clone())?
        }
        TopCommand::TEST { cses } => run_test(cses.clone())?,
    }

    Ok(())
}
