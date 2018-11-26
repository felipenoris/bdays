
use HolidayCalendar;
use chrono::Datelike;

pub mod brazil;

/// The `WeekendsOnly` holiday calendar always returns `false` for method `is_holiday`.
/// So `is_bday` method returns `false` only for weekend dates.
pub struct WeekendsOnly;

impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for WeekendsOnly {
    fn is_holiday(&self, _date: T) -> bool {
        false
    }
}
