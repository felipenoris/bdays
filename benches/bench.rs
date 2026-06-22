use bdays::HolidayCalendar;
use bdays::date::Date;
use bencher::{Bencher, benchmark_group, benchmark_main};

fn bench_weekendsonly(bench: &mut Bencher) {
    let cal = bdays::calendars::WeekendsOnly;
    let d0 = Date::from_ymd(2015, 06, 29).unwrap();
    let d1 = Date::from_ymd(2100, 12, 20).unwrap();

    bench.iter(|| {
        cal.bdays(d0, d1);
    })
}

fn bench_brsettlement(bench: &mut Bencher) {
    let cal = bdays::calendars::brazil::BRSettlement;
    let d0 = Date::from_ymd(2015, 06, 29).unwrap();
    let d1 = Date::from_ymd(2100, 12, 20).unwrap();

    bench.iter(|| {
        cal.bdays(d0, d1);
    })
}

fn bench_brsettlement_cached(bench: &mut Bencher) {
    let cal = bdays::calendars::brazil::BRSettlement;
    let d0 = Date::from_ymd(2015, 06, 29).unwrap();
    let d1 = Date::from_ymd(2100, 12, 20).unwrap();

    let cached_cal = bdays::HolidayCalendarCache::new(
        cal,
        Date::from_ymd(1980, 1, 1).unwrap(),
        Date::from_ymd(2100, 12, 31).unwrap(),
    );

    bench.iter(|| {
        cached_cal.bdays(d0, d1);
    })
}

benchmark_group!(
    benches,
    bench_weekendsonly,
    bench_brsettlement,
    bench_brsettlement_cached
);
benchmark_main!(benches);
