
use crate::HolidayCalendar;
use ::chrono::Datelike;

/// Holiday Calendars for Brazil.
pub mod brazil;

/// Holiday Calendars for the United States.
pub mod us;

/// The `WeekendsOnly` holiday calendar always returns `false` for method `is_holiday`.
/// So `is_bday` method returns `false` only for weekend dates.
pub struct WeekendsOnly;

impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for WeekendsOnly {
    fn is_holiday(&self, _date: T) -> bool {
        false
    }
}
