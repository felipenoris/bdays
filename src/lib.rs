
extern crate chrono;

use chrono::Datelike;
use chrono::Weekday;

pub trait HolidayCalendar<T: Datelike + Copy> {
    fn is_holiday(&self, date: T) -> bool;

    fn is_bday(&self, date: T) -> bool {
        !( is_weekend(date) || self.is_holiday(date) )
    }

    fn to_bday(&self, date: T, adjust_next: bool) -> T {
        let mut new_date = date;
        while !self.is_bday(new_date) {
            new_date = next_date(new_date, adjust_next);
        }
        new_date
    }

    fn advance_bdays(&self, date: T, bdays_count: i32) -> T {
        let mut new_date = self.to_bday(date, true);

        let inc_fwd = match bdays_count.signum() {
            0 => return new_date, // nothing to do
            1 => true, // bdays_count is positive
            -1 => false, // bdays_count is negative
            _ => panic!("signum function is expected to return 0, 1 or -1."),
        };

        let mut num_iterations = bdays_count.abs();

        while num_iterations > 0 {
            new_date = next_date(new_date, inc_fwd);

            // Looks for previous / next Business Day
            while !self.is_bday(new_date) {
                new_date = next_date(new_date, inc_fwd);
            }

            num_iterations += -1;
        }

        new_date
    }
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

pub struct WeekendsOnly;

impl<T: Datelike + Copy> HolidayCalendar<T> for WeekendsOnly {
    fn is_holiday(&self, _date: T) -> bool {
        false
    }
}

pub fn is_weekend<T: Datelike + Copy>(date: T) -> bool {
    match date.weekday() {
        Weekday::Sat | Weekday::Sun => true,
        _ => false
    }
}

pub mod easter;
pub mod brazil;

#[cfg(test)]
mod tests;
