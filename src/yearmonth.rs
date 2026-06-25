use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearMonthError {
    InvalidYearMonth{
        year: i32,
        month: u32,
    }
}

impl fmt::Display for YearMonthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl error::Error for YearMonthError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearMonth {
    ym: i32,
}

impl fmt::Display for YearMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y, m) = self.to_ym();

        if -10_000 < y && y < 10_000 {
            write!(f, "{y:04}-{m:02}")
        } else {
            write!(f, "{y}-{m:02}")
        }
    }
}

fn validate_yearmonth(_: i32, month: u32) -> bool {
    1 <= month && month <= 12
}

fn encode_ym(year: i32, month: u32) -> i32 {
    (year.abs() * 100 + (month as i32))*year.signum()
}

fn decode_ym(ym: i32) -> (i32, u32) {
    let s = ym.signum();
    let ym_abs = ym.abs();
    let m = ym_abs % 100;
    let y = ym_abs - m;
    (s * y / 100, m as u32)
}

impl YearMonth {

    pub fn from_ym(year: i32, month: u32) -> Result<Self, YearMonthError> {

        if !validate_yearmonth(year, month) {
            return Err(YearMonthError::InvalidYearMonth { year, month })
        }

        Ok(
            YearMonth{
                ym: encode_ym(year, month),
            }
        )
    }

    pub fn to_ym(&self) -> (i32, u32) {
        decode_ym(self.ym)
    }

    pub fn year(&self) -> i32 {
        let (yy, _) = self.to_ym();
        yy
    }

    pub fn month(&self) -> u32 {
        let (_, mm) = self.to_ym();
        mm
    }
}

#[test]
fn test_roundtrip() {
    let vals = [
        ( (2026, 1), 202601, "2026-01" ),
        ( (2026, 6), 202606, "2026-06" ),
        ( (2026, 9), 202609, "2026-09" ),
        ( (2026, 10), 202610, "2026-10" ),
        ( (2026, 11), 202611, "2026-11" ),
        ( (2026, 12), 202612, "2026-12" ),
        ( (2027, 1), 202701, "2027-01" ),
        ( (-1552, 1), -155201, "-1552-01" ),
    ];

    for (args_tuple, repr, string) in vals {
        let ym = YearMonth::from_ym(args_tuple.0, args_tuple.1).unwrap();
        assert_eq!(ym.ym, repr);
        assert_eq!(ym.to_ym(), args_tuple);
        assert_eq!(ym.year(), args_tuple.0);
        assert_eq!(ym.month(), args_tuple.1);
        assert_eq!(ym.to_string(), string);
    }
}
