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
