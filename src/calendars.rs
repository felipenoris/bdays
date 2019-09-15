
use crate::HolidayCalendar;
use ::chrono::{Datelike, NaiveDate};

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

    fn bdays(&self, d0: T, d1: T) -> i32 {

        if d0 == d1 {
            0
        } else {

            let from: NaiveDate;
            let to: NaiveDate;
            let swapped: bool;

            if d1 < d0 {
                from = NaiveDate::from_num_days_from_ce(d1.num_days_from_ce());
                to = NaiveDate::from_num_days_from_ce(d0.num_days_from_ce());
                swapped = true;
            } else {
                from = NaiveDate::from_num_days_from_ce(d0.num_days_from_ce());
                to = NaiveDate::from_num_days_from_ce(d1.num_days_from_ce());
                swapped = false;
            }

            let mut result: i32 = 0;
            let days = to.num_days_from_ce() - from.num_days_from_ce();
            let whole_weeks = days / 7;
            result += whole_weeks * 5;

            let mut current_date = NaiveDate::from_num_days_from_ce(from.num_days_from_ce() + whole_weeks * 7);

            if current_date < to {
                let mut day_of_week = current_date.weekday();

                while current_date < to {
                    if day_of_week.number_from_monday() < 6 {
                        result += 1;
                    }

                    current_date = NaiveDate::from_num_days_from_ce(current_date.num_days_from_ce() + 1);
                    day_of_week = day_of_week.succ();
                }
            }

            if swapped {
                result = -result;
            }

            result
        }
    }
}
