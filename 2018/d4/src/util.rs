use itertools::Itertools;
use time::{Date, Month, Time, PrimitiveDateTime};

use crate::INPUT;

pub fn parse_date(text: &str) -> Date {
    let tokens: (_, _, _) = text.split('-').collect_tuple().unwrap();
    let year: i32 = tokens.0.parse().unwrap();
    let month: Month = month_from_str(tokens.1).unwrap();
    let day: u8 = tokens.2.parse().unwrap();
    Date::from_calendar_date(year, month, day).unwrap()
}

pub fn month_from_str(number: &str) -> Option<Month> {
    match number {
        "01" => Some(Month::January),
        "02" => Some(Month::February),
        "03" => Some(Month::March),
        "04" => Some(Month::April),
        "05" => Some(Month::May),
        "06" => Some(Month::June),
        "07" => Some(Month::July),
        "08" => Some(Month::August),
        "09" => Some(Month::September),
        "10" => Some(Month::October),
        "11" => Some(Month::November),
        "12" => Some(Month::December),
        _ => None,
    }
}

pub fn parse_time(text: &str) -> Time {
    let tokens: (_, _) = text.split(':').collect_tuple().unwrap();
    let hour: u8 = tokens.0.parse().unwrap();
    let minute: u8 = tokens.1.parse().unwrap();
    Time::from_hms(hour, minute, 0).unwrap()
}