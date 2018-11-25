
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
