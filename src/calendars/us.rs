
use chrono::{Datelike, Weekday, NaiveDate, Duration};
use HolidayCalendar;
use easter;

/// United States federal holidays.
pub struct USSettlement;

fn end_of_month(mut yy: i32, mut mm: u32) -> NaiveDate {
    assert!(mm <= 12);

    if mm == 12 {
        yy += 1;
        mm = 1;
    } else {
        mm += 1;
    }

    NaiveDate::from_ymd(yy, mm, 1).pred()
}

#[test]
fn test_end_of_month() {
    assert_eq!( end_of_month(2018, 11), NaiveDate::from_ymd(2018, 11, 30) );
    assert_eq!( end_of_month(2018, 12), NaiveDate::from_ymd(2018, 12, 31) );
    assert_eq!( end_of_month(2019,  1), NaiveDate::from_ymd(2019,  1, 31) );
}

fn find_weekday_ascending(weekday: Weekday, yy: i32, mm: u32, occurrence: u32) -> NaiveDate {
    let anchor = NaiveDate::from_ymd(yy, mm, 1);
    let mut offset = (weekday.number_from_monday() + 7 - anchor.weekday().number_from_monday()) % 7;

    if occurrence > 1 {
        offset += 7 * (occurrence - 1);
    }

    anchor + Duration::days(offset as i64)
}

fn find_weekday_descending(weekday: Weekday, yy: i32, mm: u32, occurrence: u32) -> NaiveDate {
    let anchor = end_of_month(yy, mm);
    let mut offset = (anchor.weekday().number_from_monday() + 7 - weekday.number_from_monday()) % 7;

    if occurrence > 1 {
        offset += 7 * (occurrence - 1);
    }

    anchor - Duration::days(offset as i64)
}

fn find_weekday(weekday: Weekday, yy: i32, mm: u32, occurrence: u32, ascending: bool) -> NaiveDate {
    if ascending {
        find_weekday_ascending(weekday, yy, mm, occurrence)
    } else {
        find_weekday_descending(weekday, yy, mm, occurrence)
    }
}

#[test]
fn test_find_weekday() {
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 1, true), NaiveDate::from_ymd(2015, 07, 06));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 2, true), NaiveDate::from_ymd(2015, 07, 13));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 3, true), NaiveDate::from_ymd(2015, 07, 20));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 4, true), NaiveDate::from_ymd(2015, 07, 27));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 5, true), NaiveDate::from_ymd(2015, 08, 03));

    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 1, false), NaiveDate::from_ymd(2015, 07, 27));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 2, false), NaiveDate::from_ymd(2015, 07, 20));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 3, false), NaiveDate::from_ymd(2015, 07, 13));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 4, false), NaiveDate::from_ymd(2015, 07, 06));
    assert_eq!(find_weekday(Weekday::Mon, 2015, 07, 5, false), NaiveDate::from_ymd(2015, 06, 29));

    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 1, true), NaiveDate::from_ymd(2015, 07, 03));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 2, true), NaiveDate::from_ymd(2015, 07, 10));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 3, true), NaiveDate::from_ymd(2015, 07, 17));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 4, true), NaiveDate::from_ymd(2015, 07, 24));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 5, true), NaiveDate::from_ymd(2015, 07, 31));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 6, true), NaiveDate::from_ymd(2015, 08, 07));

    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 1, false), NaiveDate::from_ymd(2015, 07, 31));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 2, false), NaiveDate::from_ymd(2015, 07, 24));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 3, false), NaiveDate::from_ymd(2015, 07, 17));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 4, false), NaiveDate::from_ymd(2015, 07, 10));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 5, false), NaiveDate::from_ymd(2015, 07, 03));
    assert_eq!(find_weekday(Weekday::Fri, 2015, 07, 6, false), NaiveDate::from_ymd(2015, 06, 26));

    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 1, true) , NaiveDate::from_ymd(2015, 07, 01));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 2, true) , NaiveDate::from_ymd(2015, 07, 08));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 3, true) , NaiveDate::from_ymd(2015, 07, 15));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 4, true) , NaiveDate::from_ymd(2015, 07, 22));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 5, true) , NaiveDate::from_ymd(2015, 07, 29));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 6, true) , NaiveDate::from_ymd(2015, 08, 05));

    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 1, false) , NaiveDate::from_ymd(2015, 07, 29));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 2, false) , NaiveDate::from_ymd(2015, 07, 22));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 3, false) , NaiveDate::from_ymd(2015, 07, 15));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 4, false) , NaiveDate::from_ymd(2015, 07, 08));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 5, false) , NaiveDate::from_ymd(2015, 07, 01));
    assert_eq!(find_weekday(Weekday::Wed, 2015, 07, 6, false) , NaiveDate::from_ymd(2015, 06, 24));
}

/// In the United States, if a holiday falls on Saturday, it's observed on the preceding Friday.
/// If it falls on Sunday, it's observed on the next Monday.
fn adjust_weekend_holidays_us(date: NaiveDate) -> NaiveDate {
    match date.weekday() {
        Weekday::Sat => date - Duration::days(1),
        Weekday::Sun => date + Duration::days(1),
        _ => date
    }
}

impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for USSettlement {

    fn is_holiday(&self, date: T) -> bool {
        let (yy, mm, dd) = (date.year(), date.month(), date.day());
        let dt_naive = NaiveDate::from_ymd(yy, mm, dd);

        if
            // New Year's Day
            adjust_weekend_holidays_us(NaiveDate::from_ymd(yy, 1, 1)) == dt_naive
            ||
            // New Year's Day on the previous year when 1st Jan is Saturday
            (mm == 12 && dd == 31 && dt_naive.weekday() == Weekday::Fri)
            ||
            // Birthday of Martin Luther King, Jr.
            (yy >= 1983 && adjust_weekend_holidays_us(find_weekday(Weekday::Mon, yy, 1, 3, true)) == dt_naive)
            ||
            // Washington's Birthday
            adjust_weekend_holidays_us(find_weekday(Weekday::Mon, yy, 2, 3, true)) == dt_naive
            ||
            // Memorial Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Mon, yy, 5, 1, false)) == dt_naive
            ||
            // Independence Day
            adjust_weekend_holidays_us(NaiveDate::from_ymd(yy, 7, 4)) == dt_naive
            ||
            // Labor Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Mon, yy, 9, 1, true)) == dt_naive
            ||
            // Columbus Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Mon, yy, 10, 2, true)) == dt_naive
            ||
            // Veterans Day
            adjust_weekend_holidays_us(NaiveDate::from_ymd(yy, 11, 11)) == dt_naive
            ||
            // Thanksgiving Day
            adjust_weekend_holidays_us(find_weekday(Weekday::Thu, yy, 11, 4, true)) == dt_naive
            ||
            // Christmas
            adjust_weekend_holidays_us(NaiveDate::from_ymd(yy, 12, 25)) == dt_naive
         {
            return true
        }

        false
    }
}
