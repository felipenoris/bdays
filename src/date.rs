use std::error;
use std::fmt;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidDate{
        year: i32,
        month: i32,
        day: i32,
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub struct Date {
    jdn: i32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Weekday {
    Sunday = 7,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl Weekday {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Weekday::Monday),
            2 => Some(Weekday::Tuesday),
            3 => Some(Weekday::Wednesday),
            4 => Some(Weekday::Thursday),
            5 => Some(Weekday::Friday),
            6 => Some(Weekday::Saturday),
            7 => Some(Weekday::Sunday),
            _ => None,
        }
    }

    pub fn number_from_monday(self) -> i32 {
        self as i32
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

const DAYS_IN_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn days_in_month(year: i32, month: i32) -> i32 {
    if month == 2 && is_leap_year(year) {
        29
    } else {
        DAYS_IN_MONTH[(month - 1) as usize]
    }
}

fn validate_date(year: i32, month: i32, day: i32) -> bool {
    if month < 1 || month > 12 {
        return false;
    }

    day >= 1 && day <= days_in_month(year, month)
}

// Fliegel and van Flandern (1968) algorithm
// https://aa.usno.navy.mil/faq/JD_formula
fn jdn_to_ymd(jdn: i32) -> (i32, i32, i32) {
    let mut l = (jdn as i64) + 68569;
    let n = (4 * l) / 146097;
    l = l - (146097 * n + 3) / 4;
    let i = (4000 * (l + 1)) / 1461001;
    l = l - (1461 * i) / 4 + 31;
    let j = (80 * l) / 2447;

    let day = l - (2447 * j) / 80;
    l = j / 11;
    let month = j + 2 - 12 * l;
    let year = 100 * (n - 49) + i + l;

    (year as i32, month as i32, day as i32)
}

fn ymd_to_jdn(year: i32, month: i32, day: i32) -> i32 {
    let year = year as i64;
    let month = month as i64;
    let day = day as i64;

    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;

    let jdn = day
        + (153 * m + 2) / 5
        + 365 * y
        + y / 4
        - y / 100
        + y / 400
        - 32045;

    jdn.try_into().expect("Overflow")
}

impl Date {

    const JDN_COMMON_ERA_OFFSET: i32 = 1721425;

    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, Error> {

        if !validate_date(year, month, day) {
            return Err(Error::InvalidDate{year, month, day});
        }

        Ok(
            Date {
                jdn: ymd_to_jdn(year, month, day),
            }
        )
    }

    pub(crate) fn from_julian_day_number(num_days: i32) -> Self {
        Date { jdn: num_days }
    }

    pub fn from_num_days_from_ce(num_days: i32) -> Self {
        Self::from_julian_day_number(num_days + Self::JDN_COMMON_ERA_OFFSET)
    }

    pub(crate) fn julian_day_number(&self) -> i32 {
        self.jdn
    }

    pub fn advance_days(&self, days: i32) -> Self {
        Date{
            jdn: self.jdn + days,
        }
    }

    pub fn previous_date(&self) -> Self {
        self.advance_days(-1)
    }

    pub fn next_date(&self) -> Self {
        self.advance_days(1)
    }

    pub fn to_ymd(&self) -> (i32, i32, i32) {
        jdn_to_ymd(self.jdn)
    }

    pub fn num_days_from_ce(&self) -> i32 {
        self.jdn - Self::JDN_COMMON_ERA_OFFSET
    }

    pub fn weekday(&self) -> Weekday {
        let weekday_number = self.jdn % 7 + 1;

        Weekday::from_u8(
            weekday_number.try_into().unwrap(),
        ).unwrap()
    }

    pub fn start_of_month(&self) -> Self {
        let (yy, mm, _) = self.to_ymd();
        Self::from_ymd(yy, mm, 1).unwrap()
    }

    pub fn end_of_month(&self) -> Self {
        let (yy, mm, _) = self.to_ymd();
        Self::from_ymd(yy, mm, days_in_month(yy, mm)).unwrap()
    }

    pub fn year(&self) -> i32 {
        let (yy, _, _) = self.to_ymd();
        yy
    }

    pub fn month(&self) -> i32 {
        let (_, mm, _) = self.to_ymd();
        mm
    }

    pub fn day(&self) -> i32 {
        let (_, _, dd) = self.to_ymd();
        dd
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.to_ymd();

        if y < 10_000 {
            write!(f, "{y:04}-{m:02}-{d:02}")
        } else {
            write!(f, "{y}-{m:02}-{d:02}")
        }
    }
}

#[test]
fn test_sequential_jdn() {
    let mut previous_date: Option<Date> = None;

    for i in 0..5_000_000 {

        let date = Date::from_julian_day_number(i);

        let days_from_ce = date.num_days_from_ce();
        assert_eq!(date, Date::from_num_days_from_ce(days_from_ce));

        if i != 0 {
            let previous_date = previous_date.unwrap();
            assert_eq!(previous_date.num_days_from_ce(), date.num_days_from_ce() - 1);
            assert_eq!(previous_date.next_date(), date);
            assert_eq!(previous_date, date.previous_date());
        }

        let (y, m, d) = date.to_ymd();
        let date2 = Date::from_ymd(y, m, d).unwrap();
        assert_eq!(date, date2);

        previous_date = Some(date);
    }
}

impl Sub for Date {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        self.jdn - other.jdn
    }
}

#[test]
fn test_sub() {
    let dt1 = Date::from_ymd(2026, 6, 27).unwrap();
    let dt0 = Date::from_ymd(2026, 6, 26).unwrap();
    assert_eq!(dt1 - dt0, 1);
    assert_eq!(dt1 - dt1, 0);
    assert_eq!(dt0 - dt1, -1);
}

#[test]
fn test_reference_dates() {
    assert_eq!(ymd_to_jdn(2000, 1, 1), 2451545);

    let date = Date::from_ymd(2000, 1, 1).unwrap();
    assert_eq!(date.julian_day_number(), 2451545);
    assert_eq!(date.num_days_from_ce(), 730120);
    assert_eq!(date.weekday(), Weekday::Saturday);
    assert_eq!(date.weekday().number_from_monday(), 6);

    let date = Date::from_ymd(2024, 2, 29).unwrap();
    assert_eq!(date.to_ymd(), (2024, 2, 29));
    assert_eq!(date.weekday(), Weekday::Thursday);
    assert_eq!(date.weekday().number_from_monday(), 4);

    let date = Date::from_ymd(1, 1, 1).unwrap();
    assert_eq!(date.num_days_from_ce(), 1);
}

#[test]
fn test_dates_format() {
    let date = Date::from_ymd(1, 1, 1).unwrap();
    assert_eq!(date.to_string(), "0001-01-01");

    let date = Date::from_ymd(2026, 6, 22).unwrap();
    assert_eq!(date.to_string(), "2026-06-22");
}

#[test]
fn test_start_end_of_month() {
    assert_eq!( Date::from_ymd(2018, 11, 1).unwrap().end_of_month(), Date::from_ymd(2018, 11, 30).unwrap());
    assert_eq!( Date::from_ymd(2018, 12, 1).unwrap().end_of_month(), Date::from_ymd(2018, 12, 31).unwrap());
    assert_eq!( Date::from_ymd(2019, 1, 1).unwrap().end_of_month(), Date::from_ymd(2019, 1, 31).unwrap());

    assert_eq!( Date::from_ymd(2018, 11, 1).unwrap().start_of_month(), Date::from_ymd(2018, 11, 1).unwrap());
    assert_eq!( Date::from_ymd(2018, 12, 15).unwrap().start_of_month(), Date::from_ymd(2018, 12, 1).unwrap());
    assert_eq!( Date::from_ymd(2019, 1, 31).unwrap().start_of_month(), Date::from_ymd(2019, 1, 1).unwrap());
}
