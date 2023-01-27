// one page calendar

mod ct;
use ct::{print_color, print_color_bold, print_color_bold_reverse};

fn main() {
    let year = 2023;
    //let buffer = format!("One Page                                Calendar");
    //print_color_bold(&buffer, "WHITE");
    println!();
    let buffer = format!("-------------------------------------------");
    print_color_bold(&buffer, "WHITE");
    println!();
    let buffer = title_str(&format!("{}" , year));
    let buffer = center_str(&buffer, 42);

    print_color_bold(&buffer, "WHITE");
    println!();
    let buffer = format!("-------------------------------------------");
    print_color_bold(&buffer, "WHITE");
    print_month_headers(year);
    print_table();

    let buffer = format!("-------------------------------------------");
    print_color_bold(&buffer, "WHITE");
    println!();
    println!();
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

fn print_month_headers(year: i32) {
    // populate columns
    let mut sun: Vec<i32> = Vec::new();
    let mut mon: Vec<i32> = Vec::new();
    let mut tue: Vec<i32>= Vec::new();
    let mut wed: Vec<i32> = Vec::new();
    let mut thu: Vec<i32> = Vec::new();
    let mut fri: Vec<i32> = Vec::new();
    let mut sat: Vec<i32> = Vec::new();

    println!();

    for i in 1..=12 {
        match month_column(year, i) {
            0 => sun.push(i), 
            1 => mon.push(i),
            2 => tue.push(i),
            3 => wed.push(i),
            4 => thu.push(i),
            5 => fri.push(i),
            6 => sat.push(i),
            _ => println!("error")
        }
    }
    for i in 0..4 {
        print!("               ");
        if sun.len() > i {
            print_month_name(sun[i]);
        }
        else {
            print!("    ");
        }
        if mon.len() > i {
            print_month_name(mon[i]);
        }
        else {
            print!("    ");
        }
        if tue.len() > i {
            print_month_name(tue[i]);
        }
        else {
            print!("    ");
        }
        if wed.len() > i {
            print_month_name(wed[i]);
        }
        else {
            print!("    ");
        }
        if thu.len() > i {
            print_month_name(thu[i]);
        }
        else {
            print!("    ");
        }
        if fri.len() > i {
            print_month_name(fri[i]);
        }
        else {
            print!("    ");
        }
        if sat.len() > i {
            print_month_name(sat[i]);
            println!();
        }
        else {
            println!("    ");
        }
    }
}

fn print_table() {
    // get today info
    let ts = timestamp();
    let tsv: Vec<&str> = ts.split(['-', ' ']).collect();
    let year = tsv[0].parse::<i32>().unwrap();
    let _month = tsv[1].parse::<i32>().unwrap();
    let day = tsv[2].parse::<i32>().unwrap();

    let mut days: Vec<Vec<&str>> = Vec::new();
    days.push("Sun Mon Tue Wed Thu Fri Sat".split(' ').collect());
    days.push("Mon Tue Wed Thu Fri Sat Sun".split(' ').collect());
    days.push("Tue Wed Thu Fri Sat Sun Mon".split(' ').collect());
    days.push("Wed Thu Fri Sat Sun Mon Tue".split(' ').collect());
    days.push("Thu Fri Sat Sun Mon Tue Wed".split(' ').collect());
    days.push("Fri Sat Sun Mon Tue Wed Thu".split(' ').collect());
    days.push("Sat Sun Mon Tue Wed Thu Fri".split(' ').collect());

    for row in 0..7 {
        // print dates
        for col in 0..5 {
            let datecolor: &str;
            let dayval = row + 1 + col*7;

            if is_leap_year(year) {
                match dayval {
                    29 => datecolor = "MAGENTA",
                    30 => datecolor = "GREEN",
                    31 => datecolor = "BLUE",
                    _ => datecolor = "WHITE"
                }
            }
            else {
                match dayval {
                    28 => datecolor = "MAGENTA",
                    30 => datecolor = "GREEN",
                    31 => datecolor = "BLUE",
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
                if dayval == day {
                    print_color_bold_reverse(&daystring, datecolor);
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
        for d in 0..7 {
            //print!("{:>4}", days[row as usize][d]);
            let buffer = format!("{:>4}", days[row as usize][d]);
            print_color(&buffer, "GREY");
        }
        println!();
    }
}

fn print_month_name(month: i32) {
    let ts = timestamp();
    let tsv: Vec<&str> = ts.split(['-', ' ']).collect();
    //let year = tsv[0].parse::<i32>().unwrap();
    let current_month = tsv[1].parse::<i32>().unwrap();
    //let day = tsv[2].parse::<i32>().unwrap();
    //let mut monthcolor = "WHITE";
    let monthcolor = match month_name(month).as_str() {
        "JAN" => "BLUE",
        "FEB" => "MAGENTA",
        "MAR" => "BLUE",
        "APR" => "GREEN",
        "MAY" => "BLUE",
        "JUN" => "GREEN",
        "JUL" => "BLUE",
        "AUG" => "BLUE",
        "SEP" => "GREEN",
        "OCT" => "BLUE",
        "NOV" => "GREEN",
        "DEC" => "BLUE",
        _ => "WHITE"
    };

    let buffer = format!("{}", month_name(month));

    // if month is current month
    if month == current_month {
        print_color_bold_reverse(&buffer, monthcolor);
        print!(" ");
    }
    else {
        print_color(&buffer, monthcolor);
        print!(" ");
    }
}

fn month_name(month: i32) -> String {
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

fn month_column(year: i32, month: i32) -> i32 {
    day_of_week(year, month, 1)
}

fn day_of_week(year: i32, month: i32, day: i32) -> i32 {

    let mut y = year;
    let m = month;
    let d = day;
    let t: Vec<i32> = vec![0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    let dow = (y + y/4 - y/100 + y/400 + t[(m-1) as usize] + d)%7;

    dow
}

fn is_leap_year(year: i32) -> bool {
    if (year%4 == 0 && year%100 != 0) || (year%400 == 0) {
        return true;
    }
    
    false
}

fn timestamp() -> String {
    let now = chrono::Local::now();
    return now.to_string();
}
