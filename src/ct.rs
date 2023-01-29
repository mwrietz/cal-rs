use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
};

pub fn print_color(my_str: &str, color: &str) {
    execute!(
        stdout(),
        SetForegroundColor(c(color)),
        Print(my_str),
        ResetColor
    )
    .expect("print_color error");
}

pub fn print_color_bold(my_str: &str, color: &str) {
    execute!(
        stdout(),
        SetForegroundColor(c(color)),
        Print(my_str.bold()),
        ResetColor
    )
    .expect("print_color_bold error");
}

pub fn print_color_bold_reverse(my_str: &str, color: &str) {
    execute!(
        stdout(),
        SetForegroundColor(c(color)),
        Print(my_str.bold().reverse()),
        ResetColor
    )
    .expect("print_color_bold error");
}

fn c(c: &str) -> Color {
    let c_upper: &str = &c.to_uppercase();
    match c_upper {
        "RED" => Color::Red,
        "DARKRED" => Color::DarkRed,
        "BLUE" => Color::Blue,
        "DARKBLUE" => Color::DarkBlue,
        "CYAN" => Color::Cyan,
        "DARKCYAN" => Color::DarkCyan,
        "GREEN" => Color::Green,
        "DARKGREEN" => Color::DarkGreen,
        "GREY" => Color::Grey,
        "YELLOW" => Color::Yellow,
        "DARKYELLOW" => Color::DarkYellow,
        "MAGENTA" => Color::Magenta,
        "DARKMAGENTA" => Color::DarkMagenta,
        _ => Color::White,
    }
}
