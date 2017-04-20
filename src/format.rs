
pub fn format(lines: &Vec<String>, pattern: &str, color: bool) -> String {
    let mut text = lines.join("\n");
    if color {
        text = highlight(&text, pattern);
        text = highlight(&text, "*");
    }
    text
}

fn highlight(text: &str, pattern: &str) -> String {
    // let colored_pattern = color(pattern, Color::Red);
    let colored_pattern = color(&bold(pattern), Color::Red);
    text.replace(pattern, &colored_pattern)
}

#[allow(dead_code)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn color(s: &str, color: Color) -> String {
    let code = match color {
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
    };
    format!("\x1b[{}m{}\x1b[0m", code, s)
}

fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
