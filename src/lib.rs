
extern crate chrono;

use chrono::Datelike;
use chrono::Weekday;

pub trait HolidayCalendar<T: Datelike> {
    fn is_holiday(&self, date: &T) -> bool;

    fn is_bday(&self, date: &T) -> bool {
        !( is_weekend(date) || self.is_holiday(date) )
    }
}

pub struct WeekendsOnly;

impl<T: Datelike> HolidayCalendar<T> for WeekendsOnly {
    fn is_holiday(&self, _date: &T) -> bool {
        false
    }
}

pub fn is_weekend<T: Datelike>(date: &T) -> bool {
    match date.weekday() {
        Weekday::Sat | Weekday::Sun => true,
        _ => false
    }
}

pub mod easter;
pub mod brazil;

#[cfg(test)]
mod tests;
