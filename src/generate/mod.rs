use std::fs;

pub fn create_file(input: Option<String>, req_output: Option<String>) -> std::io::Result<()> {
    let input: String = input.unwrap_or("".to_string());
    let req_output: String = req_output.unwrap_or("".to_string());

    fs::copy(
        "/Users/mukeshkannan/mytemplates/template.cpp",
        "template.cpp",
    )?;
    println!("🚀 Generating template ...");
    fs::write("main.cpp", "")?;
    fs::write("req_output.txt", req_output)?;
    fs::write("input.txt", input)?;
    fs::write("output.txt", "")?;
    fs::write("debug.txt", "")?;
    println!("and Done 👍");

    Ok(())
}
