use std::str;

use std::{
    fs,
    process::{Command, Stdio},
};

use clap::{Parser, Subcommand};
use generate::create_file;
use test::compare_strings_line_by_line;

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
    TEST,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    match &args.top_command {
        TopCommand::GENERATE { input, req_output } => {
            create_file(input.clone(), req_output.clone())?
        }
        TopCommand::TEST => {
            Command::new("clang++")
                .arg("main.cpp")
                .arg("-o")
                .arg("main")
                .arg("-DONLINE_JUDGE")
                .output()
                .expect("failed to execute process");

            let cat_output = Command::new("/bin/cat")
                .arg("input.txt")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = Command::new("./main")
                .stdin(Stdio::from(cat_output.stdout.unwrap()))
                .stdout(Stdio::piped())
                .output()
                .expect("failed to execute cmd");

            let result = str::from_utf8(&output.stdout).unwrap().to_string();
            let required_result = fs::read_to_string("req_output.txt")?;

            compare_strings_line_by_line(result, required_result);
        }
    }

    Ok(())
}
