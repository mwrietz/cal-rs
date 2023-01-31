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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut today = initialize_date();

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
    // populate columns
    let mut col0: Vec<usize> = Vec::new();
    let mut col1: Vec<usize> = Vec::new();
    let mut col2: Vec<usize> = Vec::new();
    let mut col3: Vec<usize> = Vec::new();
    let mut col4: Vec<usize> = Vec::new();
    let mut col5: Vec<usize> = Vec::new();
    let mut col6: Vec<usize> = Vec::new();

    for i in 1..=12 {
        match month_column(today.calendar_year, i) {
            0 => col0.push(i),
            1 => col1.push(i),
            2 => col2.push(i),
            3 => col3.push(i),
            4 => col4.push(i),
            5 => col5.push(i),
            6 => col6.push(i),
            _ => println!("error"),
        }
    }

    let mut cols: Vec<Vec<usize>> = Vec::new();
    cols.push(col0.clone());
    cols.push(col1.clone());
    cols.push(col2.clone());
    cols.push(col3.clone());
    cols.push(col4.clone());
    cols.push(col5.clone());
    cols.push(col6.clone());

    for i in 0..3 {
        let mut buffer: String;
        if i == 1 {
            buffer = title_str(&format!("{}", today.calendar_year));
            buffer = center_str(&buffer, 15);
        } else {
            buffer = "               ".to_string();
        }

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
        print!(" ");
        println!();
    }
}

fn print_table(today: &Date) {
    let mut days: Vec<Vec<&str>> = Vec::new();
    days.push("Sun Mon Tue Wed Thu Fri Sat".split(' ').collect());
    days.push("Mon Tue Wed Thu Fri Sat Sun".split(' ').collect());
    days.push("Tue Wed Thu Fri Sat Sun Mon".split(' ').collect());
    days.push("Wed Thu Fri Sat Sun Mon Tue".split(' ').collect());
    days.push("Thu Fri Sat Sun Mon Tue Wed".split(' ').collect());
    days.push("Fri Sat Sun Mon Tue Wed Thu".split(' ').collect());
    days.push("Sat Sun Mon Tue Wed Thu Fri".split(' ').collect());

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
            let buffer = format!("{}", days[row as usize][col]);
            let daycolor: Color;
            if buffer == "Sun" {
                daycolor = Color::DarkYellow;
            } else {
                daycolor = Color::White;
            }

            if today.year == today.calendar_year
                && row == highlight_row
                && col == month_column(today.year, today.month)
            {
                print!(" ");
                print_color_bold_reverse(&buffer, daycolor);
            } else {
                print!(" ");
                print_color(&buffer, daycolor);
            }
        }
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

fn initialize_date() -> Date {
    let now = chrono::Local::now();
    let ts = now.to_string();
    let tsv: Vec<&str> = ts.split(['-', ' ']).collect();
    let today = Date {
        year: tsv[0].parse::<usize>().unwrap(),
        month: tsv[1].parse::<usize>().unwrap(),
        day: tsv[2].parse::<usize>().unwrap(),
        calendar_year: 0,
    };

    today
}

fn center_str(title: &str, width: usize) -> String {
    let pad = (width - title.len()) / 2;

    let mut buffer = String::new();
    for _i in 0..pad {
        buffer.push_str(" ");
    }
    buffer.push_str(title);
    for _i in 0..pad {
        buffer.push_str(" ");
    }

    buffer
}

fn title_str(title: &str) -> String {
    let mut buffer = String::new();
    for c in title.chars() {
        buffer.push_str(&format!("{}", c));
        buffer.push_str(" ");
    }
    buffer.pop();

    buffer
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
