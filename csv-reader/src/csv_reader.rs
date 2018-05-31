extern crate encoding;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use self::encoding::all::ISO_8859_1;
use self::encoding::{DecoderTrap, Encoding};

const FILE_NAME: &'static str = "./resources/EWR_Ortsteile_2012-12-31.csv";

pub fn get_file_content() -> Vec<Info> {
    let mut file = File::open(FILE_NAME)
        .expect(&format!("file '{}' was not found", FILE_NAME));

    let reader = BufReader::new(&mut file);

    reader.split(b'\n')
        .map(|l| l.unwrap())
        .map(|l| decode_iso(l))
        .map(|l| split_line(l))
        .map(|line| build_info(line))
        .collect()
}

fn decode_iso(unicode_chars: Vec<u8>) -> String {
    ISO_8859_1.decode(&unicode_chars, DecoderTrap::Strict).unwrap()
}

fn split_line(l: String) -> Vec<String> {
    l.split(';').map(|s|
        if starts_with(String::from(s), '"') {
            s.chars().skip(1).take(s.len() - 2).collect::<String>()
        } else {
            String::from(s)
        }
    ).collect()
}

fn starts_with(s: String, c: char) -> bool {
    s.chars().nth(0).unwrap() == c
}

fn build_info(line: Vec<String>) -> Info {
    Info {
        district_id:  line.get(0).unwrap().parse::<u8>().expect("index 0 wasn't an int"),
        district:     line.get(1).unwrap().clone(),
        city_area_id: line.get(2).unwrap().parse::<u8>().expect("index 2 wasn't an int"),
        city_area:    line.get(3).unwrap().clone(),
        gender:       if line.get(4).unwrap() == "01" { Gender::Female } else { Gender::Male },
        nationality:  if line.get(5).unwrap() == "D" { Nationality::German } else { Nationality::Other },
        age_range:    age_range(line.get(6).unwrap().clone()),
        amount:       line.get(7).unwrap().parse::<u16>().expect("index 7 wasn't an int")

    }
}

fn age_range(s: String) -> AgeRange {
    let split_values: Vec<String> = s.split('_').map(|s| s.to_string()).collect();

    let start: u8 = split_values.get(0).map(|s|
        s.chars()
            .take(2)
            .collect::<String>()
            .parse::<u8>()
            .unwrap()
    ).unwrap();

    let end = split_values.get(1).map(|s| s.parse::<u8>().unwrap());

    AgeRange(start, end)
}

pub struct Info {
    district_id: u8,
    district: String,
    city_area_id: u8,
    city_area: String,
    gender: Gender,
    nationality: Nationality,
    age_range: AgeRange,
    amount: u16
}

pub enum Gender {
    Female,
    Male
}

pub enum Nationality {
    German,
    Other
}

pub struct AgeRange(u8, Option<u8>);
