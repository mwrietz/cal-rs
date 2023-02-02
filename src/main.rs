// one page calendar

use std::env;
use std::io::{stdout, Read};
use std::process;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
};

enum Position {
    Top,
    Middle,
    Bottom,
    Side,
}

struct Date {
    year: usize,
    month: usize,
    day: usize,
    calendar_year: usize,
}

impl Default for Date {
    fn default () -> Date {
        let now = chrono::Local::now();
        let ts = now.to_string();
        let tsv: Vec<&str> = ts.split(['-', ' ']).collect();
        Date {
            year: tsv[0].parse::<usize>().unwrap(),
            month: tsv[1].parse::<usize>().unwrap(),
            day: tsv[2].parse::<usize>().unwrap(),
            calendar_year: 0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //let mut today = initialize_date();
    let mut today = Date::default();

    if args.len() < 2 {
        today.calendar_year = today.year;
    } else {
        today.calendar_year = match args[1].parse::<usize>() {
            Ok(y) => y,
            Err(_) => usage(),
        };
    }

    println!();

    line(Position::Top);
    print_month_headers(&today);
    line(Position::Middle);
    print_table(&today);
    line(Position::Bottom);
    println!();

    quit();
}

fn print_month_headers(today: &Date) {
    let mut cols: Vec<Vec<usize>> = Vec::new();
    for _i in 0..7 {
        let col: Vec<usize> = Vec::new();
        cols.push(col.clone());
    }

    for i in 1..=12 {
        cols[month_column(today.calendar_year, i)].push(i); 
    }

    for i in 0..3 {
        let buffer: String = match i {
            1 => center_str(title_str(format!("{}", today.calendar_year)), 15),
            _ => "               ".to_string()
        };

        print!(" ");
        print_color_bold(&buffer, Color::DarkYellow);
        line(Position::Side);

        print!(" ");
        for c in 0..7 {
            if cols[c].len() > i {
                print_month_name(&today, cols[c][i]);
            } else {
                print!("    ");
            }
        }
        println!(" ");
    }
}

fn title_str(title: String) -> String {
    let mut buffer = String::new();
    for c in title.chars() {
        buffer.push(c);
        buffer.push(' ');
    }
    buffer.pop();

    buffer
}

fn center_str(title: String, width: usize) -> String {
    let pad = (width - title.len()) / 2;

    let mut buffer = String::new();
    for _i in 0..pad {
        buffer.push(' ');
    }
    buffer += &title;
    for _i in 0..pad {
        buffer.push(' ');
    }

    buffer
}

fn print_table(today: &Date) {
    let mut days: Vec<&str> = "Sun Mon Tue Wed Thu Fri Sat".split(' ').collect();

    let mut highlight_row = 100;
    for row in 0..7 {
        // print dates
        print!(" ");
        for col in 0..5 {
            let dayval = row + 1 + col * 7;

            if dayval <= 31 {
                let daystring = format!("{}", dayval);

                match dayval {
                    8 | 9 => print!("  "),
                    _ => print!(" "),
                }
                if dayval == today.day && today.year == today.calendar_year {
                    print_color_bold_reverse(&daystring, date_color(dayval, today.calendar_year));
                    highlight_row = row;
                } else {
                    print_color(&daystring, date_color(dayval, today.calendar_year));
                }
            } else {
                print!("   ")
            };
        }
        print!(" ");
        line(Position::Side);

        // print days
        for col in 0..7 {
            let daycolor: Color = match days[col] {
                "Sun" => Color::DarkYellow,
                _ => Color::White
            };

            if today.year == today.calendar_year
                && row == highlight_row
                && col == month_column(today.year, today.month)
            {
                print!(" ");
                print_color_bold_reverse(days[col], daycolor);
            } else {
                print!(" ");
                print_color(days[col], daycolor);
            }
        }
        // shift days
        days.push(days[0]);
        days.remove(0);
        println!();
    }
}

fn print_month_name(today: &Date, month: usize) {
    let buffer = format!("{}", month_name(month));

    if month == today.month && today.calendar_year == today.year {
        print_color_bold_reverse(&buffer, month_color(month_name(month)));
        print!(" ");
    } else {
        print_color(&buffer, month_color(month_name(month)));
        print!(" ");
    }
}

fn month_name(month_num: usize) -> &'static str {
    match month_num {
        1 => "JAN",
        2 => "FEB",
        3 => "MAR",
        4 => "APR",
        5 => "MAY",
        6 => "JUN",
        7 => "JUL",
        8 => "AUG",
        9 => "SEP",
        10 => "OCT",
        11 => "NOV",
        12 => "DEC",
        _ => "   ",
    }
}

fn month_color(month_abbr: &str) -> Color {
    match month_abbr {
        "JAN" | "MAR" | "MAY" | "JUL" | "AUG" | "OCT" | "DEC" => Color::DarkBlue,
        "APR" | "JUN" | "SEP" | "NOV" => Color::DarkGreen,
        "FEB" => Color::DarkRed,
        _ => Color::White,
    }
}

fn date_color(dayval: usize, calendar_year: usize) -> Color {
    match dayval {
        28 => {
            if is_leap_year(calendar_year) {
                Color::White
            } else {
                Color::DarkRed
            }
        }
        29 => {
            if is_leap_year(calendar_year) {
                Color::DarkRed
            } else {
                Color::White
            }
        }
        30 => Color::DarkGreen,
        31 => Color::DarkBlue,
        _ => Color::White,
    }
}

fn month_column(year: usize, month: usize) -> usize {
    day_of_week(year, month, 1)
}

fn day_of_week(year: usize, month: usize, day: usize) -> usize {
    let mut y = year;
    let m = month;
    let d = day;
    let t: Vec<usize> = vec![0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

    if m < 3 {
        y -= 1;
    }
    let dow = (y + y / 4 - y / 100 + y / 400 + t[m - 1] + d) % 7;

    dow
}

fn is_leap_year(year: usize) -> bool {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        return true;
    }

    false
}

fn line(pos: Position) {
    let box_color = Color::White;
    let buffer_top = format!("                │                              ");
    let buffer_mid = format!("────────────────┼──────────────────────────────");
    let buffer_bot = format!("                │                              ");
    let buffer_side = format!("│");
    match pos {
        Position::Top => {
            print_color(&buffer_top, box_color);
            println!();
        }
        Position::Middle => {
            print_color(&buffer_mid, box_color);
            println!();
        }
        Position::Bottom => {
            print_color(&buffer_bot, box_color);
            println!();
        }
        Position::Side => {
            print_color(&buffer_side, box_color);
        }
    }
}

fn print_color(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str),
        ResetColor
    )
    .expect("print_color error");
}

fn print_color_bold(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str.bold()),
        ResetColor
    )
    .expect("print_color_bold error");
}

fn print_color_bold_reverse(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str.bold().reverse()),
        ResetColor
    )
    .expect("print_color_bold error");
}

fn check_version() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://raw.githubusercontent.com/mwrietz/{}/main/Cargo.toml",
        get_prog_name()
    );

    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // split body into vector of lines
    let lines: Vec<&str> = body.split("\n").collect();

    // find version in GitHub Cargo.toml
    let mut github_version = String::new();
    for line in lines {
        if line.starts_with("version") {
            github_version = line
                .replace("\"", "")
                .replace(" ", "")
                .replace("version=", "");
            break;
        }
    }

    let local_version = env!("CARGO_PKG_VERSION");

    if local_version != github_version {
        println!();
        println!(
            "The local version of '{}' is different than the GitHub version.",
            get_prog_name()
        );
        println!("    Local version  = {}", local_version);
        println!("    GitHub version = {}", github_version);
        if local_version < github_version.as_str() {
            println!("The GitHub version is newer.  Consider upgrading to the newer version.");
        } else {
            println!("The GitHub version is older.  Consider a commit.");
        }
        println!();
    }

    Ok(())
}

fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}

fn quit() {
    check_version().expect("check_version error");
    process::exit(1);
}

fn usage() -> usize {
    println!();
    print_color_bold(&get_prog_name(), Color::DarkYellow);
    println!(" v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: {} [YEAR]", get_prog_name());
    println!();

    quit();

    0
}
