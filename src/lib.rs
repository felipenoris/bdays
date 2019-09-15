
//! Provides functions to perform business days calculation between dates,
//! given a Holiday Calendar.
//!
//! A Business Day is defined as a weekday that is not a holiday.
//! To check if a date is a holiday, you must provide an implementation of the `HolidayCalendar` trait.
//!
//! This crate is a port of [BusinessDays.jl](https://github.com/felipenoris/BusinessDays.jl) to the Rust programming language.
//!
//! # Provided Holiday Calendars
//!
//! This crate provides a set of built-in holiday calendars in the `bdays::calendars` submodule.
//!
//! * `bdays::calendars::WeekendsOnly` : accounts only weekends
//! * `bdays::calendars::brazil::BRSettlement` : Brazilian banking holidays
//! * `bdays::calendars::brazil::BrazilExchange` : BMF&BOVESPA Exchange holidays (http://www.bmfbovespa.com.br)
//! * `bdays::calendars::us::USSettlement` : United States federal holidays
//!
//! # Usage
//!
//! ```rust
//! use chrono::NaiveDate;
//! use bdays::HolidayCalendar;
//!
//! // creates a holiday calendar instance
//! let cal = bdays::calendars::WeekendsOnly;
//!
//! let d0 = NaiveDate::from_ymd(2018, 11, 22);
//! let d1 = NaiveDate::from_ymd(2018, 11, 24);
//! let d2 = NaiveDate::from_ymd(2018, 11, 26);
//!
//! // checks if a date is a holiday
//! assert_eq!( cal.is_holiday(d0), false );
//!
//! // checks if a date is a business day
//! assert_eq!( cal.is_bday(d0), true  );
//! assert_eq!( cal.is_bday(d1), false );
//!
//! // adjusts to the last/next businessdays
//! assert_eq!( cal.to_bday(d1, false), NaiveDate::from_ymd(2018, 11, 23) );
//! assert_eq!( cal.to_bday(d1, true) , d2 );
//!
//! // advances a number of business days
//! assert_eq!( cal.advance_bdays(d0,  2), d2 );
//! assert_eq!( cal.advance_bdays(d2, -2), d0 );
//!
//! // returns the number of business days between dates
//! assert_eq!( cal.bdays(d0, d2),  2);
//! assert_eq!( cal.bdays(d2, d0), -2);
//! ```
//!
//! # HolidayCalendarCache
//!
//! As a motivation, this example might take some time to finish.
//!
//! ```rust
//! use chrono::NaiveDate;
//! use bdays::HolidayCalendar;
//!
//! let cal = bdays::calendars::brazil::BRSettlement;
//! let d0 = NaiveDate::from_ymd(2001, 2, 1);
//! let d1 = NaiveDate::from_ymd(2100, 2, 1);
//!
//! for _i in 0..30 {
//!     cal.bdays(d0, d1);
//! }
//! ```
//!
//! You can use `HolidayCalendarCache` to perform fast business days calculation
//! for a given range of dates.
//!
//! ```rust
//! use chrono::NaiveDate;
//! use bdays::HolidayCalendar;
//!
//! let cal = bdays::HolidayCalendarCache::new(
//!     bdays::calendars::brazil::BRSettlement,
//!     NaiveDate::from_ymd(1980, 1, 1),
//!     NaiveDate::from_ymd(2100, 12, 31)
//! );
//!
//! let d0 = NaiveDate::from_ymd(2001, 2, 1);
//! let d1 = NaiveDate::from_ymd(2100, 2, 1);
//!
//! for _i in 0..30 {
//!     cal.bdays(d0, d1);
//! }
//! ```

use chrono::Datelike;
use chrono::Weekday;

use std::fmt::Display;
use std::cmp::PartialOrd;

/// Algorithms to calculate easter dates.
pub mod easter;

/// A set of holiday calendars built into bdays crate.
pub mod calendars;

#[cfg(test)]
mod tests;

/// Returns `true` if `date` occurs on a Saturday or a Sunday.
pub fn is_weekend<T: Datelike + Copy>(date: T) -> bool {
    match date.weekday() {
        Weekday::Sat | Weekday::Sun => true,
        _ => false
    }
}

/// Returns `true` if `date` does not occur on a weekend.
pub fn is_weekday<T: Datelike + Copy>(date: T) -> bool {
    !is_weekend(date)
}

fn next_date<T: Datelike + Copy>(date: T, fwd: bool) -> T {
    let inc: i32 = {
        if fwd {
            1
        } else {
            -1
        }
    };

    match date.with_ordinal((date.ordinal() as i32 + inc) as u32) {
        Some(dt) => dt,
        None =>  {
            if fwd {
                date
                    .with_year(date.year() + 1).unwrap()
                    .with_ordinal(1).unwrap()
            } else {
                date
                    .with_year(date.year() - 1).unwrap()
                    .with_month(12).unwrap()
                    .with_day(31).unwrap()
            }
        }
    }
}

/// Abstraction for a Holiday Calendar.
pub trait HolidayCalendar<T: Datelike + Copy + PartialOrd> {

    /// Returns `true` if `date` is a holiday.
    fn is_holiday(&self, date: T) -> bool;

    /// Returns `true` if `date` is a Business Day.
    /// A Business Day is defined as a weekday that is not a holiday.
    fn is_bday(&self, date: T) -> bool {
        !(is_weekend(date) || self.is_holiday(date))
    }

    /// Adjusts `date` to the last/next business day if it's not a business day.
    fn to_bday(&self, mut date: T, adjust_next: bool) -> T {
        while !self.is_bday(date) {
            date = next_date(date, adjust_next);
        }
        date
    }

    /// Advances `bdays_count` number of business days from `date`.
    fn advance_bdays(&self, mut date: T, bdays_count: i32) -> T {
        date = self.to_bday(date, true);

        let inc_fwd = match bdays_count.signum() {
            0 => return date, // nothing to do
            1 => true, // bdays_count is positive
            -1 => false, // bdays_count is negative
            _ => panic!("signum function is expected to return 0, 1 or -1."),
        };

        let mut num_iterations = bdays_count.abs();

        while num_iterations > 0 {
            date = next_date(date, inc_fwd);

            // Looks for previous / next Business Day
            while !self.is_bday(date) {
                date = next_date(date, inc_fwd);
            }

            num_iterations += -1;
        }

        date
    }

    /// Returns the number of business days between `d0` and `d1`.
    fn bdays(&self, mut d0: T, mut d1: T) -> i32 {
        d0 = self.to_bday(d0, true);
        d1 = self.to_bday(d1, true);

        let mut from: T;
        let to: T;
        let inc: i32;
        let mut bdays_count: i32 = 0;

        if d0 <= d1 {
            inc = 1;
            from = d0;
            to = d1;
        } else {
            inc = -1;
            from = d1;
            to = d0
        }

        while from < to {
            from = self.advance_bdays(from, 1);
            bdays_count += inc;
        }

        bdays_count
    }
}

/// Caches business days calculation for a given holiday calendar
/// and a given range of dates. Implements the `HolidayCalendar` trait.
pub struct HolidayCalendarCache<T: Datelike + Copy + PartialOrd> {
    is_holiday_vec: Vec<bool>,
    is_bday_vec: Vec<bool>,
    bdays_counter_vec: Vec<i32>,
    dt_min: T,
    dt_max: T,
}

impl<T: Datelike + Copy + PartialOrd + Display> HolidayCalendarCache<T> {

    /// Creates `HolidayCalendarCache` that caches business days calculation
    /// in the range of dates from `dt_min` to `dt_max`.
    pub fn new<H: HolidayCalendar<T>>(calendar: H, dt_min: T, dt_max: T) -> HolidayCalendarCache<T> {
        if dt_min > dt_max {
            panic!("dt_min {} should not be greater than dt_max {}.", dt_min, dt_max);
        }

        let len = (dt_max.num_days_from_ce() - dt_min.num_days_from_ce() + 1) as usize;
        let mut is_holiday_vec: Vec<bool> = Vec::with_capacity(len);
        let mut is_bday_vec: Vec<bool> = Vec::with_capacity(len);
        let mut bdays_counter_vec: Vec<i32> = Vec::with_capacity(len);

        is_holiday_vec.push(calendar.is_holiday(dt_min));
        is_bday_vec.push(calendar.is_bday(dt_min));

        let mut bdays_counter = 0;
        bdays_counter_vec.push(bdays_counter);

        let mut dt = next_date(dt_min, true);
        for _i in 1..len {
            let dt_is_bday = calendar.is_bday(dt);
            is_bday_vec.push(dt_is_bday);
            is_holiday_vec.push(calendar.is_holiday(dt));

            if dt_is_bday {
                bdays_counter += 1;
            }

            bdays_counter_vec.push(bdays_counter);
            dt = next_date(dt, true);
        }

        // lengths must match
        debug_assert_eq!(is_bday_vec.len(), bdays_counter_vec.len());
        debug_assert_eq!(is_holiday_vec.len(), bdays_counter_vec.len());

        HolidayCalendarCache{
            is_holiday_vec,
            is_bday_vec,
            bdays_counter_vec,
            dt_min,
            dt_max,
        }
    }

    fn row_index(&self, date: T) -> usize {
        (date.num_days_from_ce() - self.dt_min.num_days_from_ce()) as usize
    }

    fn assert_in_bounds(&self, date: T) {
        if date < self.dt_min || self.dt_max < date {
            panic!("Date {} out of bounds of holiday calendar cache. [{}, {}].", date, self.dt_min, self.dt_max);
        }
    }
}

impl<T: Datelike + Copy + PartialOrd + Display> HolidayCalendar<T> for HolidayCalendarCache<T> {

    fn is_holiday(&self, date: T) -> bool {
        self.assert_in_bounds(date);
        self.is_holiday_vec[ self.row_index(date) ]
    }

    fn is_bday(&self, date: T) -> bool {
        self.assert_in_bounds(date);
        self.is_bday_vec[ self.row_index(date) ]
    }

    fn bdays(&self, mut d0: T, mut d1: T) -> i32 {
        d0 = self.to_bday(d0, true);
        d1 = self.to_bday(d1, true);

        self.bdays_counter_vec[ self.row_index(d1) ] - self.bdays_counter_vec[ self.row_index(d0) ]
    }
}
