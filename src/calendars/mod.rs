use crate::{HolidayCalendar, is_weekday};
use crate::date::Date;

/// Holiday Calendars for Brazil.
pub mod brazil;

/// Holiday Calendars for the United States.
pub mod us;

/// Holiday Calendars for Germany.
pub mod de;

/// The `WeekendsOnly` holiday calendar always returns `false` for method `is_holiday`.
/// So `is_bday` method returns `false` only for weekend dates.
pub struct WeekendsOnly;

impl HolidayCalendar for WeekendsOnly {

    fn is_holiday(&self, _: Date) -> bool {
        false
    }

    fn bdays(&self, d0: Date, d1: Date) -> i64 {
        if d0 == d1 {
            0
        } else {
            let from: Date;
            let to: Date;
            let swapped: bool;

            if d1 < d0 {
                from = d1;
                to = d0;
                swapped = true;
            } else {
                from = d0;
                to = d1;
                swapped = false;
            }

            let whole_weeks = (to.julian_day_number() - from.julian_day_number()) / 7;
            let mut result = whole_weeks * 5;
            let mut current_date = from.advance_days(whole_weeks * 7);

            while current_date < to {
                if is_weekday(current_date) {
                    result += 1;
                }

                current_date = current_date.next_date();
            }

            if swapped {
                result = -result;
            }

            result
        }
    }
}
