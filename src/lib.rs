#![doc = include_str!("../README.md")]

pub mod yearmonth;

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
    fn advance_bdays(&self, mut date: Date, bdays_count: i32) -> Date {
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
    fn bdays(&self, mut d0: Date, mut d1: Date) -> i32 {

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
    bdays_counter_vec: Vec<i32>,
    dt_min: Date,
    dt_max: Date,
}

impl HolidayCalendarCache {

    /// Creates `HolidayCalendarCache` that caches business days calculation
    /// in the range of dates from `dt_min` to `dt_max`.
    pub fn new<T: HolidayCalendar>(
        calendar: T,
        mut dt_min: Date,
        mut dt_max: Date,
    ) -> HolidayCalendarCache {

        if dt_min > dt_max {
            (dt_min, dt_max) = (dt_max, dt_min);
        }

        let len = (dt_max.julian_day_number() - dt_min.julian_day_number() + 1) as usize;
        let mut is_holiday_vec: Vec<bool> = Vec::with_capacity(len);
        let mut is_bday_vec: Vec<bool> = Vec::with_capacity(len);
        let mut bdays_counter_vec: Vec<i32> = Vec::with_capacity(len);

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
        (date.julian_day_number() - self.dt_min.julian_day_number()) as usize
    }

    pub fn is_date_in_bounds(&self, date: Date) -> bool {
        self.dt_min <= date && date <= self.dt_max
    }

    fn assert_in_bounds(&self, date: Date) {
        if !self.is_date_in_bounds(date) {
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

    fn bdays(&self, mut d0: Date, mut d1: Date) -> i32 {
        d0 = self.to_bday(d0, true);
        d1 = self.to_bday(d1, true);

        self.bdays_counter_vec[self.row_index(d1)] - self.bdays_counter_vec[self.row_index(d0)]
    }
}
