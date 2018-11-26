
use chrono::{Datelike, NaiveDate};
use std::fmt;
use std::error;

/// Error type for easter calculation functions.
#[derive(Debug, Clone)]
pub struct EasterError {
    y: i32
}

impl EasterError {
    fn description_string(&self) -> String {
        let mut msg = format!("Couldn't parse easter date for year {}.", self.y);

        if self.y < 1582 {
            msg.push_str(" Current algorithm only works after year 1582.");
        }

        msg
    }
}

impl fmt::Display for EasterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description_string())
    }
}

impl error::Error for EasterError {}

/// Returns easter date for year `y`
/// as the number of days since January 1, Year 1 (aka Day 1) in the proleptic Gregorian calendar.
pub fn easter_num_days_from_ce(y: i32) -> Result<i32, EasterError> {
    // Algo R only works after 1582
    if y < 1582 {
        return Err(EasterError{y})
    }

    // Century
    let c = (y / 100) + 1;

    // Shifted Epact
    let mut se = (14 + 11*(y % 19) - 3*c/4 + (5+8*c)/25 ) % 30;

    // Adjust Epact
    if (se == 0) || ((se == 1) && ( 10 < (y % 19) )) {
        se += 1;
    }

    // Paschal Moon
    let p = NaiveDate::from_ymd(y, 4, 19).num_days_from_ce() - se;

    // Easter: local the Sunday after the Paschal Moon
    Ok(p + 7 - (p % 7))
}

/// Returns easter date for year `y`
/// as a `chrono::NaiveDate`.
pub fn easter_naive_date(y: i32) -> Result<NaiveDate, EasterError> {
    let rata = easter_num_days_from_ce(y)?;

    match NaiveDate::from_num_days_from_ce_opt(rata) {
        Some(result) => Ok(result),
        None => Err(EasterError{y})
    }
}
