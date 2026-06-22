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
//! * `bdays::calendars::brazil::BrazilExchange` : B3 Exchange holidays (<https://www.b3.com.br>)
//! * `bdays::calendars::us::USSettlement` : United States federal holidays
//!
//! # Usage
//!
//! Add these dependencies to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! bdays = "0.2"
//! ```
//!
//! The following example shows the basic functions from this package.
//!
//! ```rust
//! use bdays::date::Date;
//! use bdays::HolidayCalendar;
//!
//! fn main() {
//!     // creates a holiday calendar instance
//!     let cal = bdays::calendars::WeekendsOnly;
//!
//!     let d0 = Date::from_ymd(2018, 11, 22).unwrap();
//!     let d1 = Date::from_ymd(2018, 11, 24).unwrap();
//!     let d2 = Date::from_ymd(2018, 11, 26).unwrap();
//!
//!     // checks if a date is a holiday
//!     assert_eq!( cal.is_holiday(d0), false );
//!
//!     // checks if a date is a business day
//!     assert_eq!( cal.is_bday(d0), true  );
//!     assert_eq!( cal.is_bday(d1), false );
//!
//!     // adjusts to the last/next business day
//!     assert_eq!( cal.to_bday(d1, false), Date::from_ymd(2018, 11, 23).unwrap() );
//!     assert_eq!( cal.to_bday(d1, true) , d2 );
//!
//!     // advances a number of business days
//!     assert_eq!( cal.advance_bdays(d0,  2), d2 );
//!     assert_eq!( cal.advance_bdays(d2, -2), d0 );
//!
//!     // returns the number of business days between dates
//!     assert_eq!( cal.bdays(d0, d2),  2);
//!     assert_eq!( cal.bdays(d2, d0), -2);
//! }
//! ```
//!
//! # HolidayCalendarCache
//!
//! As a motivation, this example might take some time to finish.
//!
//! ```rust
//! use bdays::date::Date;
//! use bdays::HolidayCalendar;
//!
//! let cal = bdays::calendars::brazil::BRSettlement;
//! let d0 = Date::from_ymd(2001, 2, 1).unwrap();
//! let d1 = Date::from_ymd(2100, 2, 1).unwrap();
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
//! use bdays::date::Date;
//! use bdays::HolidayCalendar;
//!
//! let cal = bdays::HolidayCalendarCache::new(
//!     bdays::calendars::brazil::BRSettlement,
//!     Date::from_ymd(1980, 1, 1).unwrap(),
//!     Date::from_ymd(2100, 12, 31).unwrap(),
//! );
//!
//! let d0 = Date::from_ymd(2001, 2, 1).unwrap();
//! let d1 = Date::from_ymd(2100, 2, 1).unwrap();
//!
//! for _i in 0..30 {
//!     cal.bdays(d0, d1);
//! }
//! ```

pub mod date;

use date::{Date, Weekday};

/// Algorithms to calculate easter dates.
pub mod easter;

/// A set of holiday calendars built into bdays crate.
pub mod calendars;

#[cfg(test)]
mod tests;

/// Returns `true` if `date` occurs on a Saturday or a Sunday.
pub fn is_weekend(date: Date) -> bool {
    matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
}

/// Returns `true` if `date` does not occur on a weekend.
pub fn is_weekday(date: Date) -> bool {
    !is_weekend(date)
}

/// Abstraction for a Holiday Calendar.
pub trait HolidayCalendar {

    /// Returns `true` if `date` is a holiday.
    fn is_holiday(&self, date: Date) -> bool;

    /// Returns `true` if `date` is a Business Day.
    /// A Business Day is defined as a weekday that is not a holiday.
    fn is_bday(&self, date: Date) -> bool {
        !(is_weekend(date) || self.is_holiday(date))
    }

    /// Adjusts `date` to the last/next business day if it's not a business day.
    fn to_bday(&self, mut date: Date, adjust_next: bool) -> Date {
        let inc = if adjust_next {
            1
        } else {
            -1
        };

        while !self.is_bday(date) {
            date = date.advance_days(inc);
        }

        date
    }

    /// Advances `bdays_count` number of business days from `date`.
    fn advance_bdays(&self, mut date: Date, bdays_count: i64) -> Date {
        date = self.to_bday(date, true);

        let inc = bdays_count.signum();

        let mut num_iterations = bdays_count.abs();

        while num_iterations > 0 {
            date = date.advance_days(inc);

            // Looks for previous / next Business Day
            while !self.is_bday(date) {
                date = date.advance_days(inc);
            }

            num_iterations += -1;
        }

        date
    }

    /// Returns the number of business days between `d0` and `d1`.
    fn bdays(&self, mut d0: Date, mut d1: Date) -> i64 {

        d0 = self.to_bday(d0, true);
        d1 = self.to_bday(d1, true);

        let mut from: Date;
        let to: Date;
        let inc;
        let mut bdays_count = 0;

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
pub struct HolidayCalendarCache {
    is_holiday_vec: Vec<bool>,
    is_bday_vec: Vec<bool>,
    bdays_counter_vec: Vec<i64>,
    dt_min: Date,
    dt_max: Date,
}

impl HolidayCalendarCache {

    /// Creates `HolidayCalendarCache` that caches business days calculation
    /// in the range of dates from `dt_min` to `dt_max`.
    pub fn new<T: HolidayCalendar>(
        calendar: T,
        dt_min: Date,
        dt_max: Date,
    ) -> HolidayCalendarCache {
        if dt_min > dt_max {
            panic!(
                "dt_min {} should not be greater than dt_max {}.",
                dt_min, dt_max
            );
        }

        let len = (dt_max.num_days_from_ce() - dt_min.num_days_from_ce() + 1) as usize;
        let mut is_holiday_vec: Vec<bool> = Vec::with_capacity(len);
        let mut is_bday_vec: Vec<bool> = Vec::with_capacity(len);
        let mut bdays_counter_vec: Vec<i64> = Vec::with_capacity(len);

        is_holiday_vec.push(calendar.is_holiday(dt_min));
        is_bday_vec.push(calendar.is_bday(dt_min));

        let mut bdays_counter = 0;
        bdays_counter_vec.push(bdays_counter);

        let mut dt = dt_min.next_date();
        for _i in 1..len {
            let dt_is_bday = calendar.is_bday(dt);
            is_bday_vec.push(dt_is_bday);
            is_holiday_vec.push(calendar.is_holiday(dt));

            if dt_is_bday {
                bdays_counter += 1;
            }

            bdays_counter_vec.push(bdays_counter);
            dt = dt.next_date();
        }

        // lengths must match
        debug_assert_eq!(is_bday_vec.len(), bdays_counter_vec.len());
        debug_assert_eq!(is_holiday_vec.len(), bdays_counter_vec.len());

        HolidayCalendarCache {
            is_holiday_vec,
            is_bday_vec,
            bdays_counter_vec,
            dt_min,
            dt_max,
        }
    }

    fn row_index(&self, date: Date) -> usize {
        (date.num_days_from_ce() - self.dt_min.num_days_from_ce()) as usize
    }

    fn assert_in_bounds(&self, date: Date) {
        if date < self.dt_min || self.dt_max < date {
            panic!(
                "Date {} out of bounds of holiday calendar cache. [{}, {}].",
                date, self.dt_min, self.dt_max
            );
        }
    }
}

impl HolidayCalendar for HolidayCalendarCache {

    fn is_holiday(&self, date: Date) -> bool {
        self.assert_in_bounds(date);
        self.is_holiday_vec[self.row_index(date)]
    }

    fn is_bday(&self, date: Date) -> bool {
        self.assert_in_bounds(date);
        self.is_bday_vec[self.row_index(date)]
    }

    fn bdays(&self, mut d0: Date, mut d1: Date) -> i64 {
        d0 = self.to_bday(d0, true);
        d1 = self.to_bday(d1, true);

        self.bdays_counter_vec[self.row_index(d1)] - self.bdays_counter_vec[self.row_index(d0)]
    }
}
