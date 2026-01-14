use regex::Regex;

// TODO: don't call twice
pub fn get_fmt_line_len(line: &str) -> usize {
    let ansi_code_pattern = Regex::new(r"\u{1b}?\[[\d+;]+m").unwrap();
    ansi_code_pattern.replace_all(line, "").chars().count()
}

fn get_longest_ln(str: &str) -> usize {
    let mut longest_line = 0;
    for line in str.lines() {
        let len = get_fmt_line_len(line);
        if len > longest_line {
            longest_line = len
        }
    }
    longest_line
}

pub fn format_ascii_art(str: String) -> String {
    let mut result = String::new();

    let longest_line = get_longest_ln(&str);

    for line in str.lines() {
        result.push_str(&format!(
            "{line}{}\n",
            " ".repeat(longest_line - get_fmt_line_len(line))
        ))
    }

    result
}
