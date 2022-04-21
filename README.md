
# bdays

[![License][license-img]](LICENSE)
[![CI][ci-img]][ci-url]
[![docs][docs-img]][docs-url]
[![version][version-img]][version-url]

[license-img]: https://img.shields.io/crates/l/bdays.svg
[ci-img]: https://github.com/felipenoris/bdays/workflows/CI/badge.svg
[ci-url]: https://github.com/felipenoris/bdays/actions
[docs-img]: https://docs.rs/bdays/badge.svg
[docs-url]: https://docs.rs/bdays
[version-img]: https://img.shields.io/crates/v/bdays.svg
[version-url]: https://crates.io/crates/bdays

Provides functions to perform business days calculation between dates,
given a Holiday Calendar.

A Business Day is defined as a weekday that is not a holiday.

To check if a date is a holiday, you must provide an implementation of the `HolidayCalendar` trait.

This crate is a port of [BusinessDays.jl](https://github.com/felipenoris/BusinessDays.jl) to the Rust programming language.

## Provided Holiday Calendars

This crate provides a set of built-in holiday calendars in the `bdays::calendars` submodule.

* `bdays::calendars::WeekendsOnly` : accounts only weekends

* `bdays::calendars::brazil::BRSettlement` : Brazilian banking holidays

* `bdays::calendars::brazil::BrazilExchange` : B3 Exchange holidays (http://www.b3.com.br)

* `bdays::calendars::us::USSettlement` : United States federal holidays

## Usage

Add these dependencies to your `Cargo.toml` file.

```toml
[dependencies]
bdays = "0.1"
chrono = "0.4"
```

The following example shows the basic functions from this package.

```rust
use chrono::NaiveDate;
use bdays::HolidayCalendar;

fn main() {
    // creates a holiday calendar instance
    let cal = bdays::calendars::WeekendsOnly;

    let d0 = NaiveDate::from_ymd(2018, 11, 22);
    let d1 = NaiveDate::from_ymd(2018, 11, 24);
    let d2 = NaiveDate::from_ymd(2018, 11, 26);

    // checks if a date is a holiday
    assert_eq!( cal.is_holiday(d0), false );

    // checks if a date is a business day
    assert_eq!( cal.is_bday(d0), true  );
    assert_eq!( cal.is_bday(d1), false );

    // adjusts to the last/next business day
    assert_eq!( cal.to_bday(d1, false), NaiveDate::from_ymd(2018, 11, 23) );
    assert_eq!( cal.to_bday(d1, true) , d2 );

    // advances a number of business days
    assert_eq!( cal.advance_bdays(d0,  2), d2 );
    assert_eq!( cal.advance_bdays(d2, -2), d0 );

    // returns the number of business days between dates
    assert_eq!( cal.bdays(d0, d2),  2);
    assert_eq!( cal.bdays(d2, d0), -2);
}
```
## HolidayCalendarCache

As a motivation, this example might take some time to finish.
```rust
use chrono::NaiveDate;
use bdays::HolidayCalendar;

let cal = bdays::calendars::brazil::BRSettlement;
let d0 = NaiveDate::from_ymd(2001, 2, 1);
let d1 = NaiveDate::from_ymd(2100, 2, 1);

for _i in 0..30 {
    cal.bdays(d0, d1);
}
```
You can use `HolidayCalendarCache` to perform fast business days calculation
for a given range of dates.

```rust
use chrono::NaiveDate;
use bdays::HolidayCalendar;

let cal = bdays::HolidayCalendarCache::new(
    bdays::calendars::brazil::BRSettlement,
    NaiveDate::from_ymd(1980, 1, 1),
    NaiveDate::from_ymd(2100, 12, 31)
);

let d0 = NaiveDate::from_ymd(2001, 2, 1);
let d1 = NaiveDate::from_ymd(2100, 2, 1);

for _i in 0..30 {
    cal.bdays(d0, d1);
}
```
