
#[macro_use]
extern crate bencher;
extern crate bdays;
extern crate chrono;

use bdays::HolidayCalendar;
use bencher::Bencher;

fn bench_brsettlement(bench: &mut Bencher) {

    let cal = bdays::brazil::BRSettlement;
    let d0 = chrono::NaiveDate::from_ymd(2015, 06, 29);
    let d1 = chrono::NaiveDate::from_ymd(2100, 12, 20);

    bench.iter(|| {
        cal.bdays(d0, d1);
    })
}

benchmark_group!(benches, bench_brsettlement);
benchmark_main!(benches);
