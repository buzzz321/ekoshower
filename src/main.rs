use chrono::{Datelike, NaiveDate};
use core::num;
use lexical::{parse_with_options, ParseFloatOptions};
use std::env;
use std::{fs, process};
//use chrono::format::ParseError;

#[derive(Debug)]
struct Item {
    date: NaiveDate,
    description: String,
    amount: f64,
    category: String,
}

impl Item {
    fn new(date: NaiveDate, description: String, amount: f64, category: String) -> Self {
        Self {
            date,
            description,
            amount,
            category,
        }
    }
}

fn process_lines(data: &str) -> Vec<Item> {
    //    #[cfg(all(feature = "parse_floats", feature = "format"))]
    const EUROPEAN: u128 = lexical::NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b' '))
        .build();
    let options = ParseFloatOptions::builder()
        .decimal_point(b',')
        .build()
        .unwrap();
    let items: Vec<Item> = data
        .lines()
        .map(|line| {
            let tmp: Vec<&str>;
            let line_fixed: String;
            if line.contains(r#"""#) {
                // This looks ugly but since i need to replace the ; inside all quoted strings
                // and replace will allocate a new string which feels wastefull.
                // would be nice to be able to reuse the .split part between string and &str branch.
                line_fixed = line.to_string().replace("&amp;", "&");
                tmp = line_fixed
                    .split(";")
                    .enumerate()
                    .filter(|&(i, _)| i == 2 || i == 3 || i == 6 || i == 8)
                    .map(|(_, part)| part)
                    .collect::<Vec<&str>>();
            } else {
                tmp = line
                    .split(";")
                    .enumerate()
                    .filter(|&(i, _)| i == 3 || i == 4 || i == 6 || i == 8)
                    .map(|(_, part)| part)
                    .collect::<Vec<&str>>();
            }
            let str_dig = tmp[2].replace("\u{a0}", "");
            let amount = parse_with_options::<f64, _, EUROPEAN>(str_dig, &options).unwrap();
            let date_only = NaiveDate::parse_from_str(tmp[0], "%Y-%m-%d").unwrap();
            //println!("{:?}", date_only.month());
            Item::new(date_only, tmp[1].to_string(), amount, tmp[3].to_string())
        })
        .collect();
    items
}

fn sum_category_month(data: &[Item], category: &str, month: u32) -> f64 {
    let mut retval:f64 = 0.0;
    for item in data {
        if (*item).category == category.to_owned() && (*item).date.month() == month {
            retval += (*item).amount.abs();
        }
    }
    retval
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?} {:?}", args, args.len());

    if args.len() != 2 {
        println!("To few arguments needs a filename to work!");
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let items = process_lines(&contents);

    println!("Sum food for May {:?}", sum_category_month(&items, "Mat", 5));
    println!("Sum car for May {:?}", sum_category_month(&items, "Bil", 5));
}
