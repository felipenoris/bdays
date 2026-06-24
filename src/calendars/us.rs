use crate::date::{Date, Weekday};
use crate::HolidayCalendar;

/// United States federal holidays.
pub struct USSettlement;

fn find_weekday(target_weekday: Weekday, yy: i32, mm: u32, occurrence: u32, ascending: bool) -> Date {

    assert!(occurrence > 0);

    let mut anchor = Date::from_ymd(yy, mm, 1).unwrap();
    let mut offset: i32;

    if ascending {
        offset = ( target_weekday.number_from_monday() + 7 - anchor.weekday().number_from_monday() ) % 7;
    } else {
        anchor = anchor.end_of_month();
        offset = ( anchor.weekday().number_from_monday() + 7 - target_weekday.number_from_monday() ) % 7;
    }

    offset += (occurrence as i32 - 1) * 7;

    if ascending {
        anchor.advance_days(offset)
    } else {
        anchor.advance_days(-offset)
    }
}

#[test]
fn test_find_weekday() {
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 1, true), Date::from_ymd(2015, 07, 06).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 2, true), Date::from_ymd(2015, 07, 13).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 3, true), Date::from_ymd(2015, 07, 20).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 4, true), Date::from_ymd(2015, 07, 27).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 5, true), Date::from_ymd(2015, 08, 03).expect("Valid date"));

    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 1, false), Date::from_ymd(2015, 07, 27).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 2, false), Date::from_ymd(2015, 07, 20).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 3, false), Date::from_ymd(2015, 07, 13).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 4, false), Date::from_ymd(2015, 07, 06).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Monday, 2015, 07, 5, false), Date::from_ymd(2015, 06, 29).expect("Valid date"));

    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 1, true), Date::from_ymd(2015, 07, 03).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 2, true), Date::from_ymd(2015, 07, 10).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 3, true), Date::from_ymd(2015, 07, 17).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 4, true), Date::from_ymd(2015, 07, 24).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 5, true), Date::from_ymd(2015, 07, 31).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 6, true), Date::from_ymd(2015, 08, 07).expect("Valid date"));

    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 1, false), Date::from_ymd(2015, 07, 31).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 2, false), Date::from_ymd(2015, 07, 24).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 3, false), Date::from_ymd(2015, 07, 17).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 4, false), Date::from_ymd(2015, 07, 10).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 5, false), Date::from_ymd(2015, 07, 03).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Friday, 2015, 07, 6, false), Date::from_ymd(2015, 06, 26).expect("Valid date"));

    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 1, true) , Date::from_ymd(2015, 07, 01).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 2, true) , Date::from_ymd(2015, 07, 08).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 3, true) , Date::from_ymd(2015, 07, 15).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 4, true) , Date::from_ymd(2015, 07, 22).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 5, true) , Date::from_ymd(2015, 07, 29).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 6, true) , Date::from_ymd(2015, 08, 05).expect("Valid date"));

    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 1, false) , Date::from_ymd(2015, 07, 29).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 2, false) , Date::from_ymd(2015, 07, 22).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 3, false) , Date::from_ymd(2015, 07, 15).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 4, false) , Date::from_ymd(2015, 07, 08).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 5, false) , Date::from_ymd(2015, 07, 01).expect("Valid date"));
    assert_eq!(find_weekday(Weekday::Wednesday, 2015, 07, 6, false) , Date::from_ymd(2015, 06, 24).expect("Valid date"));
}

/// In the United States, if a holiday falls on Saturday, it's observed on the preceding Friday.
/// If it falls on Sunday, it's observed on the next Monday.
fn adjust_weekend_holidays_us(date: Date) -> Date {
    match date.weekday() {
        Weekday::Saturday => date.previous_date(),
        Weekday::Sunday => date.next_date(),
        _ => date
    }
}

impl HolidayCalendar for USSettlement {

    fn is_holiday(&self, date: Date) -> bool {
        let (yy, mm, dd) = date.to_ymd();

        if
            // New Year's Day
            adjust_weekend_holidays_us(Date::from_ymd(yy, 1, 1).expect("Valid date")) == date
            ||
            // New Year's Day on the previous year when 1st Jan is Saturday
            (mm == 12 && dd == 31 && date.weekday() == Weekday::Friday)
            ||
            // Birthday of Martin Luther King, Jr.
            (yy >= 1983 && adjust_weekend_holidays_us(find_weekday(Weekday::Monday, yy, 1, 3, true)) == date)
            ||
            // Washington's Birthday
            adjust_weekend_holidays_us(find_weekday(Weekday::Monday, yy, 2, 3, true)) == date
            ||
            // Memorial Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Monday, yy, 5, 1, false)) == date
            ||
            // Juneteenth
            (yy >= 2021 && adjust_weekend_holidays_us(Date::from_ymd(yy, 6, 19).expect("Valid date")) == date )
            ||
            // Independence Day
            adjust_weekend_holidays_us(Date::from_ymd(yy, 7, 4).expect("Valid date")) == date
            ||
            // Labor Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Monday, yy, 9, 1, true)) == date
            ||
            // Columbus Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Monday, yy, 10, 2, true)) == date
            ||
            // Veterans Day
            adjust_weekend_holidays_us(Date::from_ymd(yy, 11, 11).expect("Valid date")) == date
            ||
            // Thanksgiving Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Thursday, yy, 11, 4, true)) == date
            ||
            // Christmas
            adjust_weekend_holidays_us(Date::from_ymd(yy, 12, 25).expect("Valid date")) == date
         {
            return true
        }

        false
    }
}
