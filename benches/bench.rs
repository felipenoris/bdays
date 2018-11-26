
#[macro_use]
extern crate bencher;
extern crate bdays;
extern crate chrono;

use bdays::HolidayCalendar;
use bencher::Bencher;

fn bench_weekendsonly(bench: &mut Bencher) {

    let cal = bdays::calendars::WeekendsOnly;
    let d0 = chrono::NaiveDate::from_ymd(2015, 06, 29);
    let d1 = chrono::NaiveDate::from_ymd(2100, 12, 20);

    bench.iter(|| {
        cal.bdays(d0, d1);
    })
}

fn bench_brsettlement(bench: &mut Bencher) {

    let cal = bdays::calendars::brazil::BRSettlement;
    let d0 = chrono::NaiveDate::from_ymd(2015, 06, 29);
    let d1 = chrono::NaiveDate::from_ymd(2100, 12, 20);

    bench.iter(|| {
        cal.bdays(d0, d1);
    })
}

fn bench_brsettlement_cached(bench: &mut Bencher) {

    let cal = bdays::calendars::brazil::BRSettlement;
    let d0 = chrono::NaiveDate::from_ymd(2015, 06, 29);
    let d1 = chrono::NaiveDate::from_ymd(2100, 12, 20);

    let cached_cal = bdays::HolidayCalendarCache::new(cal, chrono::NaiveDate::from_ymd(1980, 1, 1), chrono::NaiveDate::from_ymd(2100, 12, 31));

    bench.iter(|| {
        cached_cal.bdays(d0, d1);
    })
}

benchmark_group!(benches, bench_weekendsonly, bench_brsettlement, bench_brsettlement_cached);
benchmark_main!(benches);
