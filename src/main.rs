// one page calendar

mod ct;
mod gh_repo_status;

use ct::{print_color, print_color_bold, print_color_bold_reverse};
use std::env;
use std::process;

struct Date {
    year: usize,
    month: usize,
    day: usize,
    calendar_year: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let ts = timestamp();
    let tsv: Vec<&str> = ts.split(['-', ' ']).collect();

    let mut today = Date {
        year:  tsv[0].parse::<usize>().unwrap(),
        month: tsv[1].parse::<usize>().unwrap(),
        day: tsv[2].parse::<usize>().unwrap(),
        calendar_year: 0
    };

    if args.len() < 2 {
        today.calendar_year = today.year;
    }
    else {
        today.calendar_year = match args[1].parse::<usize>() {
            Ok(y) => y,
            Err(_) => {
                usage()
            }
        };
    }

    println!();
    hline();

    let buffer = title_str(&format!("{}" , today.calendar_year));
    let buffer = center_str(&buffer, 42);
    print_color_bold(&buffer, "DARKYELLOW");
    println!();

    hline();

    print_month_headers(&today);
    print_table(&today);

    hline();
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
            _ => println!("error")
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
        print!("               ");
        for c in 0..7 {
            if cols[c].len() > i {
                print_month_name(&today, cols[c][i]);
            }
            else {
                print!("    ");
            }
        }
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
        for col in 0..5 {
            let datecolor: &str;
            let dayval = row + 1 + col*7;

            if is_leap_year(today.calendar_year) {
                match dayval {
                    29 => datecolor = "DARKMAGENTA",
                    30 => datecolor = "DARKGREEN",
                    31 => datecolor = "DARKBLUE",
                    _ => datecolor = "WHITE"
                }
            }
            else {
                match dayval {
                    28 => datecolor = "DARKMAGENTA",
                    30 => datecolor = "DARKGREEN",
                    31 => datecolor = "DARKBLUE",
                    _ => datecolor = "WHITE"
                }
            }

            if dayval <= 31 {
                let daystring = format!("{}", dayval);
                if dayval < 8 {
                    print!(" ");
                }
                else {
                    if dayval < 10 {
                        print!("  ");
                    }
                    else {
                        print!(" ");
                    }
                }
                if dayval == today.day {
                    if today.year == today.calendar_year {
                        print_color_bold_reverse(&daystring, datecolor);
                    }
                    else {
                        print_color(&daystring, datecolor);
                    }
                    highlight_row = row;
                }
                else {
                    print_color(&daystring, datecolor);
                }
            }
            else {
                print!("   ")
            };
        }

        // print days
        for col in 0..7 {
            let buffer = format!("{}", days[row as usize][col]);
            let daycolor: &str;
            if buffer == "Sun" {
                daycolor = "DARKRED"
            }
            else {
                daycolor = "WHITE"
            }

            if row == highlight_row {
                if today.year == today.calendar_year {
                    if col == month_column(today.year, today.month) { 
                        print!(" ");
                        print_color_bold_reverse(&buffer, daycolor);
                    }
                    else {
                        print!(" ");
                        print_color(&buffer, daycolor);
                    }
                }
                else {
                    print!(" ");
                    print_color(&buffer, daycolor);
                }
//                }
            }
            else {
                print!(" ");
                print_color(&buffer, daycolor);
            }
        }
        println!();
    }
}

fn print_month_name(today: &Date, month: usize) {
    let monthcolor = match month_name(month).as_str() {
        "JAN" => "DARKBLUE",
        "FEB" => "DARKMAGENTA",
        "MAR" => "DARKBLUE",
        "APR" => "DARKGREEN",
        "MAY" => "DARKBLUE",
        "JUN" => "DARKGREEN",
        "JUL" => "DARKBLUE",
        "AUG" => "DARKBLUE",
        "SEP" => "DARKGREEN",
        "OCT" => "DARKBLUE",
        "NOV" => "DARKGREEN",
        "DEC" => "DARKBLUE",
        _ => "WHITE"
    };

    let buffer = format!("{}", month_name(month));

    // if month is current month
    if month == today.month && today.calendar_year == today.year {
        print_color_bold_reverse(&buffer, monthcolor);
        print!(" ");
    }
    else {
        print_color(&buffer, monthcolor);
        print!(" ");
    }
}

fn month_name(month: usize) -> String {
    match month {
       1 => String::from("JAN"),
       2 => String::from("FEB"),
       3 => String::from("MAR"),
       4 => String::from("APR"),
       5 => String::from("MAY"),
       6 => String::from("JUN"),
       7 => String::from("JUL"),
       8 => String::from("AUG"),
       9 => String::from("SEP"),
       10 => String::from("OCT"),
       11 => String::from("NOV"),
       12 => String::from("DEC"),
       _ => String::from("   ")
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
    let dow = (y + y/4 - y/100 + y/400 + t[m-1] + d)%7;

    dow
}

fn is_leap_year(year: usize) -> bool {
    if (year%4 == 0 && year%100 != 0) || (year%400 == 0) {
        return true;
    }

    false
}

fn timestamp() -> String {
    let now = chrono::Local::now();
    return now.to_string();
}

fn center_str(title: &str, width: usize) -> String {
    let pad = (width - title.len())/2;

    let mut buffer = String::new();
    for _i in 0..pad {
        buffer.push_str(" ");
    }
    buffer.push_str(title);

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

fn hline() {
    let buffer = format!("-------------------------------------------");
    print_color(&buffer, "WHITE");
    println!();
}

fn usage() -> usize {
    println!();
    //println!("{} v{}", get_prog_name(), env!("CARGO_PKG_VERSION"));
    print_color_bold(&get_prog_name(), "YELLOW");
    println!(" v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: {} [YEAR]", get_prog_name());
    println!();

    quit();

    0
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
    gh_repo_status::check_version()
        .expect("check_version error");
    process::exit(1);
}
