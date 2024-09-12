use std::{
    fs, io,
    path::PathBuf,
    process::{Command, Output, Stdio},
    str,
};

use clap::error::Result;
use colored::Colorize;

pub fn compare_strings_line_by_line(string1: String, string2: String) {
    let lines1: Vec<&str> = string1.lines().collect();
    let lines2: Vec<&str> = string2.lines().collect();

    let max_lines = std::cmp::max(lines1.len(), lines2.len());

    for i in 0..max_lines {
        let line1 = lines1.get(i).unwrap_or(&"");
        let line2 = lines2.get(i).unwrap_or(&"");

        if line1 != line2 {
            println!("{}", format!("{} {}", line1, line2).on_bright_red().black())
        } else {
            println!(
                "{}",
                format!("{} {}", line1, line2).on_bright_green().black()
            )
        }
    }
}

pub fn run_test(cses: Option<String>) -> std::io::Result<()> {
    Command::new("clang++")
        .arg("main.cpp")
        .arg("-o")
        .arg("main")
        .arg("-DONLINE_JUDGE")
        .output()
        .expect("failed to execute process");

    match cses {
        Some(_) => {
            let entries = fs::read_dir("./tests")?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<PathBuf>, io::Error>>()?
                .into_iter()
                .filter(|x| match x.extension() {
                    Some(x) => x == "in",
                    None => false,
                })
                .collect::<Vec<_>>();

            for elm in entries {
                println!("{:?}", elm.to_str())
            }
        }
        None => {
            let output = test_file("input.txt");

            let result = str::from_utf8(&output.stdout).unwrap().to_string();
            let required_result = fs::read_to_string("req_output.txt")?;

            compare_strings_line_by_line(result, required_result);
        }
    }

    Ok(())
}

fn test_file(file: &str) -> Output {
    let cat_output = Command::new("/bin/cat")
        .arg(file)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("./main")
        .stdin(Stdio::from(cat_output.stdout.unwrap()))
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute cmd");

    output
}
