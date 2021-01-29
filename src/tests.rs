
use super::{HolidayCalendar, easter, calendars, HolidayCalendarCache};
use chrono::{NaiveDate, Datelike};

#[test]
fn test_next_date() {
    assert_eq!(super::next_date(NaiveDate::from_ymd(2018, 11, 23), true), NaiveDate::from_ymd(2018, 11, 24));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2018, 11, 23), false), NaiveDate::from_ymd(2018, 11, 22));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2018, 12, 31), true), NaiveDate::from_ymd(2019, 1, 1));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2018, 12, 31), false), NaiveDate::from_ymd(2018, 12, 30));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2019, 1, 1), true), NaiveDate::from_ymd(2019, 1, 2));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2019, 1, 1), false), NaiveDate::from_ymd(2018, 12, 31));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2018, 11, 30), true), NaiveDate::from_ymd(2018, 12, 1));
    assert_eq!(super::next_date(NaiveDate::from_ymd(2018, 12, 1), false), NaiveDate::from_ymd(2018, 11, 30));
}

#[test]
fn test_weekday() {
    assert!(super::is_weekday(NaiveDate::from_ymd(2018, 11, 26)));
    assert!(super::is_weekday(NaiveDate::from_ymd(2018, 11, 27)));
    assert!(super::is_weekday(NaiveDate::from_ymd(2018, 11, 28)));
    assert!(super::is_weekday(NaiveDate::from_ymd(2018, 11, 29)));
    assert!(super::is_weekday(NaiveDate::from_ymd(2018, 11, 30)));
    assert!(!super::is_weekday(NaiveDate::from_ymd(2018, 12, 01)));
    assert!(!super::is_weekday(NaiveDate::from_ymd(2018, 12, 02)));
}

#[test]
fn test_weekend() {
    assert!(!super::is_weekend(NaiveDate::from_ymd(2018, 11, 26)));
    assert!(!super::is_weekend(NaiveDate::from_ymd(2018, 11, 27)));
    assert!(!super::is_weekend(NaiveDate::from_ymd(2018, 11, 28)));
    assert!(!super::is_weekend(NaiveDate::from_ymd(2018, 11, 29)));
    assert!(!super::is_weekend(NaiveDate::from_ymd(2018, 11, 30)));
    assert!(super::is_weekend(NaiveDate::from_ymd(2018, 12, 01)));
    assert!(super::is_weekend(NaiveDate::from_ymd(2018, 12, 02)));
}

fn weekend_calendar_tests<H: HolidayCalendar<NaiveDate>>(cal: H) {

    {
        let dt = NaiveDate::from_ymd(2018, 11, 23);
        assert_eq!(cal.is_bday(dt), true);
    }

    {
        let dt = NaiveDate::from_ymd(2018, 11, 24);
        assert_eq!(cal.is_bday(dt), false);
    }

    {
        let dt = NaiveDate::from_ymd(2018, 11, 25);
        assert_eq!(cal.is_bday(dt), false)
    }

    {
        let dt = NaiveDate::from_ymd(2018, 11, 26);
        assert_eq!(cal.is_bday(dt), true)
    }

    {
        let d0 = NaiveDate::from_ymd(2016, 9, 25);
        let d1 = NaiveDate::from_ymd(2016, 9, 28);
        assert_eq!(cal.bdays(d0, d1), 2);
        assert_eq!(cal.bdays(d1, d0), -2);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 9, 2);
        assert_eq!(cal.bdays(d0, d1), 5);
        assert_eq!(cal.bdays(d1, d0), -5);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 9, 3);
        assert_eq!(cal.bdays(d0, d1), 6);
        assert_eq!(cal.bdays(d1, d0), -6);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 9, 9);
        assert_eq!(cal.bdays(d0, d1), 10);
        assert_eq!(cal.bdays(d1, d0), -10);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 9, 10);
        assert_eq!(cal.bdays(d0, d1), 11);
        assert_eq!(cal.bdays(d1, d0), -11);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 8, 30);
        assert_eq!(cal.bdays(d0, d1), 4);
        assert_eq!(cal.bdays(d1, d0), -4);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 8, 27);
        assert_eq!(cal.bdays(d0, d1), 1);
        assert_eq!(cal.bdays(d1, d0), -1);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 8, 26);
        assert_eq!(cal.bdays(d0, d1), 0);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 8, 19);
        assert_eq!(cal.bdays(d0, d1), -5);
        assert_eq!(cal.bdays(d1, d0), 5);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 26);
        let d1 = NaiveDate::from_ymd(2019, 8, 25);
        assert_eq!(cal.bdays(d0, d1), 0);
        assert_eq!(cal.bdays(d1, d0), 0);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 24);
        let d1 = NaiveDate::from_ymd(2019, 8, 25);
        assert_eq!(cal.bdays(d0, d1), 0);
        assert_eq!(cal.bdays(d1, d0), 0);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 24);
        let d1 = NaiveDate::from_ymd(2019, 8, 26);
        assert_eq!(cal.bdays(d0, d1), 0);
        assert_eq!(cal.bdays(d1, d0), 0);
    }

    {
        let d0 = NaiveDate::from_ymd(2019, 8, 23);
        let d1 = NaiveDate::from_ymd(2019, 8, 24);
        assert_eq!(cal.bdays(d0, d1), 1);
        assert_eq!(cal.bdays(d1, d0), -1);
    }
}

#[test]
fn test_weekend_calendar_no_cache() {
    let cal = calendars::WeekendsOnly;
    weekend_calendar_tests(cal);
}

#[test]
fn test_weekend_calendar_cached() {
    let dt_min = NaiveDate::from_ymd(2015, 1, 1);
    let dt_max = NaiveDate::from_ymd(2025, 1, 1);
    let cal = HolidayCalendarCache::new(calendars::WeekendsOnly, dt_min, dt_max);
    weekend_calendar_tests(cal);
}

#[test]
fn test_arith() {
    assert_eq!(3 / 2, 1);
    assert_eq!(3.0 / 2.0, 1.5);
    assert_eq!(5 % 2, 1);
}

#[test]
fn test_days_from_ce() {
    assert_eq!( NaiveDate::from_ymd(1900, 4, 19).num_days_from_ce(), 693704);
}

#[test]
fn test_easter() {
    assert_eq!(easter::easter_naive_date(1901).unwrap(), NaiveDate::from_ymd(1901, 04, 07));
    assert_eq!(easter::easter_naive_date(1902).unwrap(), NaiveDate::from_ymd(1902, 03, 30));
    assert_eq!(easter::easter_naive_date(1903).unwrap(), NaiveDate::from_ymd(1903, 04, 12));
    assert_eq!(easter::easter_naive_date(1904).unwrap(), NaiveDate::from_ymd(1904, 04, 03));
    assert_eq!(easter::easter_naive_date(1905).unwrap(), NaiveDate::from_ymd(1905, 04, 23));
    assert_eq!(easter::easter_naive_date(1906).unwrap(), NaiveDate::from_ymd(1906, 04, 15));
    assert_eq!(easter::easter_naive_date(1907).unwrap(), NaiveDate::from_ymd(1907, 03, 31));
    assert_eq!(easter::easter_naive_date(1908).unwrap(), NaiveDate::from_ymd(1908, 04, 19));
    assert_eq!(easter::easter_naive_date(1909).unwrap(), NaiveDate::from_ymd(1909, 04, 11));
    assert_eq!(easter::easter_naive_date(1910).unwrap(), NaiveDate::from_ymd(1910, 03, 27));
    assert_eq!(easter::easter_naive_date(1911).unwrap(), NaiveDate::from_ymd(1911, 04, 16));
    assert_eq!(easter::easter_naive_date(1912).unwrap(), NaiveDate::from_ymd(1912, 04, 07));
    assert_eq!(easter::easter_naive_date(1913).unwrap(), NaiveDate::from_ymd(1913, 03, 23));
    assert_eq!(easter::easter_naive_date(1914).unwrap(), NaiveDate::from_ymd(1914, 04, 12));
    assert_eq!(easter::easter_naive_date(1915).unwrap(), NaiveDate::from_ymd(1915, 04, 04));
    assert_eq!(easter::easter_naive_date(1916).unwrap(), NaiveDate::from_ymd(1916, 04, 23));
    assert_eq!(easter::easter_naive_date(1917).unwrap(), NaiveDate::from_ymd(1917, 04, 08));
    assert_eq!(easter::easter_naive_date(1918).unwrap(), NaiveDate::from_ymd(1918, 03, 31));
    assert_eq!(easter::easter_naive_date(1919).unwrap(), NaiveDate::from_ymd(1919, 04, 20));
    assert_eq!(easter::easter_naive_date(1920).unwrap(), NaiveDate::from_ymd(1920, 04, 04));
    assert_eq!(easter::easter_naive_date(1921).unwrap(), NaiveDate::from_ymd(1921, 03, 27));
    assert_eq!(easter::easter_naive_date(1922).unwrap(), NaiveDate::from_ymd(1922, 04, 16));
    assert_eq!(easter::easter_naive_date(1923).unwrap(), NaiveDate::from_ymd(1923, 04, 01));
    assert_eq!(easter::easter_naive_date(1924).unwrap(), NaiveDate::from_ymd(1924, 04, 20));
    assert_eq!(easter::easter_naive_date(1925).unwrap(), NaiveDate::from_ymd(1925, 04, 12));
    assert_eq!(easter::easter_naive_date(1926).unwrap(), NaiveDate::from_ymd(1926, 04, 04));
    assert_eq!(easter::easter_naive_date(1927).unwrap(), NaiveDate::from_ymd(1927, 04, 17));
    assert_eq!(easter::easter_naive_date(1928).unwrap(), NaiveDate::from_ymd(1928, 04, 08));
    assert_eq!(easter::easter_naive_date(1929).unwrap(), NaiveDate::from_ymd(1929, 03, 31));
    assert_eq!(easter::easter_naive_date(1930).unwrap(), NaiveDate::from_ymd(1930, 04, 20));
    assert_eq!(easter::easter_naive_date(1931).unwrap(), NaiveDate::from_ymd(1931, 04, 05));
    assert_eq!(easter::easter_naive_date(1932).unwrap(), NaiveDate::from_ymd(1932, 03, 27));
    assert_eq!(easter::easter_naive_date(1933).unwrap(), NaiveDate::from_ymd(1933, 04, 16));
    assert_eq!(easter::easter_naive_date(1934).unwrap(), NaiveDate::from_ymd(1934, 04, 01));
    assert_eq!(easter::easter_naive_date(1935).unwrap(), NaiveDate::from_ymd(1935, 04, 21));
    assert_eq!(easter::easter_naive_date(1936).unwrap(), NaiveDate::from_ymd(1936, 04, 12));
    assert_eq!(easter::easter_naive_date(1937).unwrap(), NaiveDate::from_ymd(1937, 03, 28));
    assert_eq!(easter::easter_naive_date(1938).unwrap(), NaiveDate::from_ymd(1938, 04, 17));
    assert_eq!(easter::easter_naive_date(1939).unwrap(), NaiveDate::from_ymd(1939, 04, 09));
    assert_eq!(easter::easter_naive_date(1940).unwrap(), NaiveDate::from_ymd(1940, 03, 24));
    assert_eq!(easter::easter_naive_date(1941).unwrap(), NaiveDate::from_ymd(1941, 04, 13));
    assert_eq!(easter::easter_naive_date(1942).unwrap(), NaiveDate::from_ymd(1942, 04, 05));
    assert_eq!(easter::easter_naive_date(1943).unwrap(), NaiveDate::from_ymd(1943, 04, 25));
    assert_eq!(easter::easter_naive_date(1944).unwrap(), NaiveDate::from_ymd(1944, 04, 09));
    assert_eq!(easter::easter_naive_date(1945).unwrap(), NaiveDate::from_ymd(1945, 04, 01));
    assert_eq!(easter::easter_naive_date(1946).unwrap(), NaiveDate::from_ymd(1946, 04, 21));
    assert_eq!(easter::easter_naive_date(1947).unwrap(), NaiveDate::from_ymd(1947, 04, 06));
    assert_eq!(easter::easter_naive_date(1948).unwrap(), NaiveDate::from_ymd(1948, 03, 28));
    assert_eq!(easter::easter_naive_date(1949).unwrap(), NaiveDate::from_ymd(1949, 04, 17));
    assert_eq!(easter::easter_naive_date(1950).unwrap(), NaiveDate::from_ymd(1950, 04, 09));
    assert_eq!(easter::easter_naive_date(1951).unwrap(), NaiveDate::from_ymd(1951, 03, 25));
    assert_eq!(easter::easter_naive_date(1952).unwrap(), NaiveDate::from_ymd(1952, 04, 13));
    assert_eq!(easter::easter_naive_date(1953).unwrap(), NaiveDate::from_ymd(1953, 04, 05));
    assert_eq!(easter::easter_naive_date(1954).unwrap(), NaiveDate::from_ymd(1954, 04, 18));
    assert_eq!(easter::easter_naive_date(1955).unwrap(), NaiveDate::from_ymd(1955, 04, 10));
    assert_eq!(easter::easter_naive_date(1956).unwrap(), NaiveDate::from_ymd(1956, 04, 01));
    assert_eq!(easter::easter_naive_date(1957).unwrap(), NaiveDate::from_ymd(1957, 04, 21));
    assert_eq!(easter::easter_naive_date(1958).unwrap(), NaiveDate::from_ymd(1958, 04, 06));
    assert_eq!(easter::easter_naive_date(1959).unwrap(), NaiveDate::from_ymd(1959, 03, 29));
    assert_eq!(easter::easter_naive_date(1960).unwrap(), NaiveDate::from_ymd(1960, 04, 17));
    assert_eq!(easter::easter_naive_date(1961).unwrap(), NaiveDate::from_ymd(1961, 04, 02));
    assert_eq!(easter::easter_naive_date(1962).unwrap(), NaiveDate::from_ymd(1962, 04, 22));
    assert_eq!(easter::easter_naive_date(1963).unwrap(), NaiveDate::from_ymd(1963, 04, 14));
    assert_eq!(easter::easter_naive_date(1964).unwrap(), NaiveDate::from_ymd(1964, 03, 29));
    assert_eq!(easter::easter_naive_date(1965).unwrap(), NaiveDate::from_ymd(1965, 04, 18));
    assert_eq!(easter::easter_naive_date(1966).unwrap(), NaiveDate::from_ymd(1966, 04, 10));
    assert_eq!(easter::easter_naive_date(1967).unwrap(), NaiveDate::from_ymd(1967, 03, 26));
    assert_eq!(easter::easter_naive_date(1968).unwrap(), NaiveDate::from_ymd(1968, 04, 14));
    assert_eq!(easter::easter_naive_date(1969).unwrap(), NaiveDate::from_ymd(1969, 04, 06));
    assert_eq!(easter::easter_naive_date(1970).unwrap(), NaiveDate::from_ymd(1970, 03, 29));
    assert_eq!(easter::easter_naive_date(1971).unwrap(), NaiveDate::from_ymd(1971, 04, 11));
    assert_eq!(easter::easter_naive_date(1972).unwrap(), NaiveDate::from_ymd(1972, 04, 02));
    assert_eq!(easter::easter_naive_date(1973).unwrap(), NaiveDate::from_ymd(1973, 04, 22));
    assert_eq!(easter::easter_naive_date(1974).unwrap(), NaiveDate::from_ymd(1974, 04, 14));
    assert_eq!(easter::easter_naive_date(1975).unwrap(), NaiveDate::from_ymd(1975, 03, 30));
    assert_eq!(easter::easter_naive_date(1976).unwrap(), NaiveDate::from_ymd(1976, 04, 18));
    assert_eq!(easter::easter_naive_date(1977).unwrap(), NaiveDate::from_ymd(1977, 04, 10));
    assert_eq!(easter::easter_naive_date(1978).unwrap(), NaiveDate::from_ymd(1978, 03, 26));
    assert_eq!(easter::easter_naive_date(1979).unwrap(), NaiveDate::from_ymd(1979, 04, 15));
    assert_eq!(easter::easter_naive_date(1980).unwrap(), NaiveDate::from_ymd(1980, 04, 06));
    assert_eq!(easter::easter_naive_date(1981).unwrap(), NaiveDate::from_ymd(1981, 04, 19));
    assert_eq!(easter::easter_naive_date(1982).unwrap(), NaiveDate::from_ymd(1982, 04, 11));
    assert_eq!(easter::easter_naive_date(1983).unwrap(), NaiveDate::from_ymd(1983, 04, 03));
    assert_eq!(easter::easter_naive_date(1984).unwrap(), NaiveDate::from_ymd(1984, 04, 22));
    assert_eq!(easter::easter_naive_date(1985).unwrap(), NaiveDate::from_ymd(1985, 04, 07));
    assert_eq!(easter::easter_naive_date(1986).unwrap(), NaiveDate::from_ymd(1986, 03, 30));
    assert_eq!(easter::easter_naive_date(1987).unwrap(), NaiveDate::from_ymd(1987, 04, 19));
    assert_eq!(easter::easter_naive_date(1988).unwrap(), NaiveDate::from_ymd(1988, 04, 03));
    assert_eq!(easter::easter_naive_date(1989).unwrap(), NaiveDate::from_ymd(1989, 03, 26));
    assert_eq!(easter::easter_naive_date(1990).unwrap(), NaiveDate::from_ymd(1990, 04, 15));
    assert_eq!(easter::easter_naive_date(1991).unwrap(), NaiveDate::from_ymd(1991, 03, 31));
    assert_eq!(easter::easter_naive_date(1992).unwrap(), NaiveDate::from_ymd(1992, 04, 19));
    assert_eq!(easter::easter_naive_date(1993).unwrap(), NaiveDate::from_ymd(1993, 04, 11));
    assert_eq!(easter::easter_naive_date(1994).unwrap(), NaiveDate::from_ymd(1994, 04, 03));
    assert_eq!(easter::easter_naive_date(1995).unwrap(), NaiveDate::from_ymd(1995, 04, 16));
    assert_eq!(easter::easter_naive_date(1996).unwrap(), NaiveDate::from_ymd(1996, 04, 07));
    assert_eq!(easter::easter_naive_date(1997).unwrap(), NaiveDate::from_ymd(1997, 03, 30));
    assert_eq!(easter::easter_naive_date(1998).unwrap(), NaiveDate::from_ymd(1998, 04, 12));
    assert_eq!(easter::easter_naive_date(1999).unwrap(), NaiveDate::from_ymd(1999, 04, 04));
    assert_eq!(easter::easter_naive_date(2000).unwrap(), NaiveDate::from_ymd(2000, 04, 23));
    assert_eq!(easter::easter_naive_date(2001).unwrap(), NaiveDate::from_ymd(2001, 04, 15));
    assert_eq!(easter::easter_naive_date(2002).unwrap(), NaiveDate::from_ymd(2002, 03, 31));
    assert_eq!(easter::easter_naive_date(2003).unwrap(), NaiveDate::from_ymd(2003, 04, 20));
    assert_eq!(easter::easter_naive_date(2004).unwrap(), NaiveDate::from_ymd(2004, 04, 11));
    assert_eq!(easter::easter_naive_date(2005).unwrap(), NaiveDate::from_ymd(2005, 03, 27));
    assert_eq!(easter::easter_naive_date(2006).unwrap(), NaiveDate::from_ymd(2006, 04, 16));
    assert_eq!(easter::easter_naive_date(2007).unwrap(), NaiveDate::from_ymd(2007, 04, 08));
    assert_eq!(easter::easter_naive_date(2008).unwrap(), NaiveDate::from_ymd(2008, 03, 23));
    assert_eq!(easter::easter_naive_date(2009).unwrap(), NaiveDate::from_ymd(2009, 04, 12));
    assert_eq!(easter::easter_naive_date(2010).unwrap(), NaiveDate::from_ymd(2010, 04, 04));
    assert_eq!(easter::easter_naive_date(2011).unwrap(), NaiveDate::from_ymd(2011, 04, 24));
    assert_eq!(easter::easter_naive_date(2012).unwrap(), NaiveDate::from_ymd(2012, 04, 08));
    assert_eq!(easter::easter_naive_date(2013).unwrap(), NaiveDate::from_ymd(2013, 03, 31));
    assert_eq!(easter::easter_naive_date(2014).unwrap(), NaiveDate::from_ymd(2014, 04, 20));
    assert_eq!(easter::easter_naive_date(2015).unwrap(), NaiveDate::from_ymd(2015, 04, 05));
    assert_eq!(easter::easter_naive_date(2016).unwrap(), NaiveDate::from_ymd(2016, 03, 27));
    assert_eq!(easter::easter_naive_date(2017).unwrap(), NaiveDate::from_ymd(2017, 04, 16));
    assert_eq!(easter::easter_naive_date(2018).unwrap(), NaiveDate::from_ymd(2018, 04, 01));
    assert_eq!(easter::easter_naive_date(2019).unwrap(), NaiveDate::from_ymd(2019, 04, 21));
    assert_eq!(easter::easter_naive_date(2020).unwrap(), NaiveDate::from_ymd(2020, 04, 12));
    assert_eq!(easter::easter_naive_date(2021).unwrap(), NaiveDate::from_ymd(2021, 04, 04));
    assert_eq!(easter::easter_naive_date(2022).unwrap(), NaiveDate::from_ymd(2022, 04, 17));
    assert_eq!(easter::easter_naive_date(2023).unwrap(), NaiveDate::from_ymd(2023, 04, 09));
    assert_eq!(easter::easter_naive_date(2024).unwrap(), NaiveDate::from_ymd(2024, 03, 31));
    assert_eq!(easter::easter_naive_date(2025).unwrap(), NaiveDate::from_ymd(2025, 04, 20));
    assert_eq!(easter::easter_naive_date(2026).unwrap(), NaiveDate::from_ymd(2026, 04, 05));
    assert_eq!(easter::easter_naive_date(2027).unwrap(), NaiveDate::from_ymd(2027, 03, 28));
    assert_eq!(easter::easter_naive_date(2028).unwrap(), NaiveDate::from_ymd(2028, 04, 16));
    assert_eq!(easter::easter_naive_date(2029).unwrap(), NaiveDate::from_ymd(2029, 04, 01));
    assert_eq!(easter::easter_naive_date(2030).unwrap(), NaiveDate::from_ymd(2030, 04, 21));
    assert_eq!(easter::easter_naive_date(2031).unwrap(), NaiveDate::from_ymd(2031, 04, 13));
    assert_eq!(easter::easter_naive_date(2032).unwrap(), NaiveDate::from_ymd(2032, 03, 28));
    assert_eq!(easter::easter_naive_date(2033).unwrap(), NaiveDate::from_ymd(2033, 04, 17));
    assert_eq!(easter::easter_naive_date(2034).unwrap(), NaiveDate::from_ymd(2034, 04, 09));
    assert_eq!(easter::easter_naive_date(2035).unwrap(), NaiveDate::from_ymd(2035, 03, 25));
    assert_eq!(easter::easter_naive_date(2036).unwrap(), NaiveDate::from_ymd(2036, 04, 13));
    assert_eq!(easter::easter_naive_date(2037).unwrap(), NaiveDate::from_ymd(2037, 04, 05));
    assert_eq!(easter::easter_naive_date(2038).unwrap(), NaiveDate::from_ymd(2038, 04, 25));
    assert_eq!(easter::easter_naive_date(2039).unwrap(), NaiveDate::from_ymd(2039, 04, 10));
    assert_eq!(easter::easter_naive_date(2040).unwrap(), NaiveDate::from_ymd(2040, 04, 01));
    assert_eq!(easter::easter_naive_date(2041).unwrap(), NaiveDate::from_ymd(2041, 04, 21));
    assert_eq!(easter::easter_naive_date(2042).unwrap(), NaiveDate::from_ymd(2042, 04, 06));
    assert_eq!(easter::easter_naive_date(2043).unwrap(), NaiveDate::from_ymd(2043, 03, 29));
    assert_eq!(easter::easter_naive_date(2044).unwrap(), NaiveDate::from_ymd(2044, 04, 17));
    assert_eq!(easter::easter_naive_date(2045).unwrap(), NaiveDate::from_ymd(2045, 04, 09));
    assert_eq!(easter::easter_naive_date(2046).unwrap(), NaiveDate::from_ymd(2046, 03, 25));
    assert_eq!(easter::easter_naive_date(2047).unwrap(), NaiveDate::from_ymd(2047, 04, 14));
    assert_eq!(easter::easter_naive_date(2048).unwrap(), NaiveDate::from_ymd(2048, 04, 05));
    assert_eq!(easter::easter_naive_date(2049).unwrap(), NaiveDate::from_ymd(2049, 04, 18));
    assert_eq!(easter::easter_naive_date(2050).unwrap(), NaiveDate::from_ymd(2050, 04, 10));
    assert_eq!(easter::easter_naive_date(2051).unwrap(), NaiveDate::from_ymd(2051, 04, 02));
    assert_eq!(easter::easter_naive_date(2052).unwrap(), NaiveDate::from_ymd(2052, 04, 21));
    assert_eq!(easter::easter_naive_date(2053).unwrap(), NaiveDate::from_ymd(2053, 04, 06));
    assert_eq!(easter::easter_naive_date(2054).unwrap(), NaiveDate::from_ymd(2054, 03, 29));
    assert_eq!(easter::easter_naive_date(2055).unwrap(), NaiveDate::from_ymd(2055, 04, 18));
    assert_eq!(easter::easter_naive_date(2056).unwrap(), NaiveDate::from_ymd(2056, 04, 02));
    assert_eq!(easter::easter_naive_date(2057).unwrap(), NaiveDate::from_ymd(2057, 04, 22));
    assert_eq!(easter::easter_naive_date(2058).unwrap(), NaiveDate::from_ymd(2058, 04, 14));
    assert_eq!(easter::easter_naive_date(2059).unwrap(), NaiveDate::from_ymd(2059, 03, 30));
    assert_eq!(easter::easter_naive_date(2060).unwrap(), NaiveDate::from_ymd(2060, 04, 18));
    assert_eq!(easter::easter_naive_date(2061).unwrap(), NaiveDate::from_ymd(2061, 04, 10));
    assert_eq!(easter::easter_naive_date(2062).unwrap(), NaiveDate::from_ymd(2062, 03, 26));
    assert_eq!(easter::easter_naive_date(2063).unwrap(), NaiveDate::from_ymd(2063, 04, 15));
    assert_eq!(easter::easter_naive_date(2064).unwrap(), NaiveDate::from_ymd(2064, 04, 06));
    assert_eq!(easter::easter_naive_date(2065).unwrap(), NaiveDate::from_ymd(2065, 03, 29));
    assert_eq!(easter::easter_naive_date(2066).unwrap(), NaiveDate::from_ymd(2066, 04, 11));
    assert_eq!(easter::easter_naive_date(2067).unwrap(), NaiveDate::from_ymd(2067, 04, 03));
    assert_eq!(easter::easter_naive_date(2068).unwrap(), NaiveDate::from_ymd(2068, 04, 22));
    assert_eq!(easter::easter_naive_date(2069).unwrap(), NaiveDate::from_ymd(2069, 04, 14));
    assert_eq!(easter::easter_naive_date(2070).unwrap(), NaiveDate::from_ymd(2070, 03, 30));
    assert_eq!(easter::easter_naive_date(2071).unwrap(), NaiveDate::from_ymd(2071, 04, 19));
    assert_eq!(easter::easter_naive_date(2072).unwrap(), NaiveDate::from_ymd(2072, 04, 10));
    assert_eq!(easter::easter_naive_date(2073).unwrap(), NaiveDate::from_ymd(2073, 03, 26));
    assert_eq!(easter::easter_naive_date(2074).unwrap(), NaiveDate::from_ymd(2074, 04, 15));
    assert_eq!(easter::easter_naive_date(2075).unwrap(), NaiveDate::from_ymd(2075, 04, 07));
    assert_eq!(easter::easter_naive_date(2076).unwrap(), NaiveDate::from_ymd(2076, 04, 19));
    assert_eq!(easter::easter_naive_date(2077).unwrap(), NaiveDate::from_ymd(2077, 04, 11));
    assert_eq!(easter::easter_naive_date(2078).unwrap(), NaiveDate::from_ymd(2078, 04, 03));
}

fn br_settlement_tests<H: HolidayCalendar<NaiveDate>>(cal: H) {
    // Brazil HolidayCalendar tests

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2014, 12, 31)), true); // wednesday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 01, 01)), false); // new year
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 01, 02)), true); // friday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 04, 20)), true); // monday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 04, 21)), false); // tiradentes
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 04, 22)), true); // wednesday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 04, 30)), true); // thursday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 05, 01)), false); // labor day
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 05, 02)), false); // saturday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 09, 06)), false); // sunday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 09, 07)), false); // independence day
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 09, 08)), true); // tuesday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 10, 11)), false); // sunday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 10, 12)), false); // Nossa Senhora Aparecida
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 10, 13)), true); // tuesday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 11, 01)), false); // sunday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 11, 02)), false); // Finados
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2015, 11, 03)), true); // tuesday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 11, 14)), true); // thursday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 11, 15)), false); // Republic
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 11, 16)), false); // saturday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 12, 24)), true); // tuesday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 12, 25)), false); // Christmas
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 12, 26)), true); // thursday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 02, 08)), true); // friday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 02, 09)), false); // saturday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 02, 10)), false); // sunday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 02, 11)), false); // segunda carnaval
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 02, 12)), false); // terca carnaval
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 02, 13)), true); // wednesday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 03, 28)), true); // thursday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 03, 29)), false); // sexta-feira santa
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 03, 30)), false); // saturday

    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 05, 29)), true); // wednesday
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 05, 30)), false); // Corpus Christi
    assert_eq!(cal.is_bday(NaiveDate::from_ymd(2013, 05, 31)), true); // friday
}

#[test]
fn test_br_settlement_no_cache() {
    let cal = calendars::brazil::BRSettlement;
    br_settlement_tests(cal);
}

#[test]
fn test_br_settlement_cached() {
    let d0 = NaiveDate::from_ymd(2010, 1, 1);
    let d1 = NaiveDate::from_ymd(2017, 1, 1);
    let cal = HolidayCalendarCache::new(calendars::brazil::BRSettlement, d0, d1);
    br_settlement_tests(cal);
}

#[test]
fn test_to_bday() {
    let cal = calendars::brazil::BRSettlement;
    assert_eq!(cal.to_bday(NaiveDate::from_ymd(2013, 02, 08), true), NaiveDate::from_ymd(2013, 02, 08)); // regular friday
    assert_eq!(cal.to_bday(NaiveDate::from_ymd(2013, 02, 08), false), NaiveDate::from_ymd(2013, 02, 08)); // regular friday
    assert_eq!(cal.to_bday(NaiveDate::from_ymd(2013, 02, 09), true), NaiveDate::from_ymd(2013, 02, 13)); // after carnaval
    assert_eq!(cal.to_bday(NaiveDate::from_ymd(2013, 02, 13), false), NaiveDate::from_ymd(2013, 02, 13)); // after carnaval
    assert_eq!(cal.to_bday(NaiveDate::from_ymd(2013, 02, 12), false), NaiveDate::from_ymd(2013, 02, 08)); // before carnaval
}

#[test]
fn test_advance_bdays() {
    let cal = calendars::brazil::BRSettlement;
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 0), NaiveDate::from_ymd(2013, 02, 06)); // regular wednesday
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 1), NaiveDate::from_ymd(2013, 02, 07)); // regular thursday
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 07), -1), NaiveDate::from_ymd(2013, 02, 06)); // regular thursday
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 2), NaiveDate::from_ymd(2013, 02, 08)); // regular friday
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 3), NaiveDate::from_ymd(2013, 02, 13)); // after carnaval wednesday
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 4), NaiveDate::from_ymd(2013, 02, 14)); // after carnaval thursday
    assert_eq!(cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 14), -4), NaiveDate::from_ymd(2013, 02, 06)); // after carnaval thursday
}

#[test]
fn test_bdays() {
    let cal = calendars::brazil::BRSettlement;

    {
        let d0 = NaiveDate::from_ymd(2018, 11, 26);
        let d1 = NaiveDate::from_ymd(2018, 11, 27);
        assert_eq!(cal.bdays(d0, d0), 0);
        assert_eq!(cal.bdays(d0, d1), 1);
        assert_eq!(cal.bdays(d1, d0), -1);
    }

    {
        let d0 = NaiveDate::from_ymd(2018, 11, 25);
        let d1 = NaiveDate::from_ymd(2018, 11, 27);
        assert_eq!(cal.bdays(d0, d0), 0);
        assert_eq!(cal.bdays(d0, d1), 1);
        assert_eq!(cal.bdays(d1, d0), -1);
    }

    {
        let d0 = NaiveDate::from_ymd(2018, 11, 23);
        let d1 = NaiveDate::from_ymd(2018, 11, 27);
        assert_eq!(cal.bdays(d0, d0), 0);
        assert_eq!(cal.bdays(d0, d1), 2);
        assert_eq!(cal.bdays(d1, d0), -2);
    }

    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 06)), 0);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 07)), 1);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 07), NaiveDate::from_ymd(2013, 02, 06)), -1);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 08)), 2);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 08), NaiveDate::from_ymd(2013, 02, 06)), -2);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 09)), 3);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 10)), 3);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 11)), 3);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 12)), 3);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 13)), 3);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 14)), 4);
    assert_eq!(cal.bdays(NaiveDate::from_ymd(2013, 02, 14), NaiveDate::from_ymd(2013, 02, 06)), -4);
}

#[test]
fn test_holiday_calendar_cache() {
    let uncached_cal = calendars::brazil::BRSettlement;

    let d0 = NaiveDate::from_ymd(1980, 1, 1);
    let d1 = NaiveDate::from_ymd(2100, 12, 31);
    let cached_cal = HolidayCalendarCache::new(calendars::brazil::BRSettlement, d0, d1);

    let mut dt = d0;
    while dt <= d1 {
        assert_eq!(cached_cal.is_bday(dt), uncached_cal.is_bday(dt));
        assert_eq!(cached_cal.is_holiday(dt), uncached_cal.is_holiday(dt));
        dt = NaiveDate::from_num_days_from_ce(dt.num_days_from_ce() + 1);
    }

    // to_bday
    assert_eq!(cached_cal.to_bday(NaiveDate::from_ymd(2013, 02, 08), true), NaiveDate::from_ymd(2013, 02, 08)); // regular friday
    assert_eq!(cached_cal.to_bday(NaiveDate::from_ymd(2013, 02, 08), false), NaiveDate::from_ymd(2013, 02, 08)); // regular friday
    assert_eq!(cached_cal.to_bday(NaiveDate::from_ymd(2013, 02, 09), true), NaiveDate::from_ymd(2013, 02, 13)); // after carnaval
    assert_eq!(cached_cal.to_bday(NaiveDate::from_ymd(2013, 02, 13), false), NaiveDate::from_ymd(2013, 02, 13)); // after carnaval
    assert_eq!(cached_cal.to_bday(NaiveDate::from_ymd(2013, 02, 12), false), NaiveDate::from_ymd(2013, 02, 08)); // before carnaval

    // advance_bdays
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 0), NaiveDate::from_ymd(2013, 02, 06)); // regular wednesday
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 1), NaiveDate::from_ymd(2013, 02, 07)); // regular thursday
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 07), -1), NaiveDate::from_ymd(2013, 02, 06)); // regular thursday
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 2), NaiveDate::from_ymd(2013, 02, 08)); // regular friday
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 3), NaiveDate::from_ymd(2013, 02, 13)); // after carnaval wednesday
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 06), 4), NaiveDate::from_ymd(2013, 02, 14)); // after carnaval thursday
    assert_eq!(cached_cal.advance_bdays(NaiveDate::from_ymd(2013, 02, 14), -4), NaiveDate::from_ymd(2013, 02, 06)); // after carnaval thursday

    // bdays
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 06)), 0);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 07)), 1);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 07), NaiveDate::from_ymd(2013, 02, 06)), -1);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 08)), 2);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 08), NaiveDate::from_ymd(2013, 02, 06)), -2);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 09)), 3);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 10)), 3);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 11)), 3);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 12)), 3);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 13)), 3);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 06), NaiveDate::from_ymd(2013, 02, 14)), 4);
    assert_eq!(cached_cal.bdays(NaiveDate::from_ymd(2013, 02, 14), NaiveDate::from_ymd(2013, 02, 06)), -4);
    assert_eq!(cached_cal.bdays(d0, d1), uncached_cal.bdays(d0, d1));
}

#[test]
#[should_panic(expected="out of bounds of holiday calendar cache")]
fn test_holiday_calendar_cache_is_bday_panic() {
    let d0 = NaiveDate::from_ymd(1980, 1, 1);
    let d1 = NaiveDate::from_ymd(2100, 12, 31);
    let cached_cal = HolidayCalendarCache::new(calendars::brazil::BRSettlement, d0, d1);
    cached_cal.is_bday(NaiveDate::from_ymd(2101, 1, 1));
}

#[test]
#[should_panic(expected="out of bounds of holiday calendar cache")]
fn test_holiday_calendar_cache_bdays_panic_1() {
    let d0 = NaiveDate::from_ymd(1980, 1, 1);
    let d1 = NaiveDate::from_ymd(2100, 12, 31);
    let cached_cal = HolidayCalendarCache::new(calendars::brazil::BRSettlement, d0, d1);
    cached_cal.bdays(NaiveDate::from_ymd(2000, 1, 1), NaiveDate::from_ymd(2101, 1, 1));
}

#[test]
#[should_panic(expected="out of bounds of holiday calendar cache")]
fn test_holiday_calendar_cache_bdays_panic_2() {
    let d0 = NaiveDate::from_ymd(1980, 1, 1);
    let d1 = NaiveDate::from_ymd(2100, 12, 31);
    let cached_cal = HolidayCalendarCache::new(calendars::brazil::BRSettlement, d0, d1);
    cached_cal.bdays(NaiveDate::from_ymd(1970, 1, 1), NaiveDate::from_ymd(2000, 1, 1));
}

fn us_settlement_tests<H: HolidayCalendar<NaiveDate>>(us: H) {
    // Federal Holidays listed on https://www.opm.gov/policy-data-oversight/snow-dismissal-procedures/federal-holidays/#url=2015

    assert!(us.is_bday(NaiveDate::from_ymd(2014, 12, 31)));
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 1, 1))); // New Year's Day - Thursday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 1, 2)));

    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 1, 18)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 1, 18))); // Sunday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 1, 19))); // Birthday of Martin Luther King, Jr. - Monday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 1, 20)));

    assert!(us.is_bday(NaiveDate::from_ymd(1982, 1, 18))); // not a holiday for Martin Luther King

    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 2, 15)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 2, 15))); // Sunday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 2, 16))); // Washington’s Birthday - Monday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 2, 17)));

    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 5, 24)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 5, 24))); // Sunday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 5, 25))); // Memorial Day - Monday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 5, 26)));

    assert!(us.is_bday(NaiveDate::from_ymd(2015, 7, 2)));
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 7, 3))); // Independence Day - Friday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 7, 4)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 7, 4))); // Saturday

    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 9, 6)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 9, 6))); // Sunday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 9, 7))); // Labor Day - Monday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 9, 8)));

    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 10, 11)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 10, 11))); // Sunday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 10, 12))); // Columbus Day - Monday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 10, 13)));

    assert!(us.is_bday(NaiveDate::from_ymd(2015, 11, 10)));
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 11, 11))); // Veterans Day - Wednesday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 11, 12)));

    assert!(us.is_bday(NaiveDate::from_ymd(2015, 11, 25)));
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 11, 26))); // Thanksgiving Day - Thursday
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 11, 27)));

    assert!(us.is_bday(NaiveDate::from_ymd(2015, 11, 25)));
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 11, 26))); // Thanksgiving Day - Thursday
    assert!(us.is_holiday(NaiveDate::from_ymd(2015, 11, 26)));
    assert!(us.is_bday(NaiveDate::from_ymd(2015, 11, 27)));

    assert!(us.is_bday(NaiveDate::from_ymd(2015, 12, 24)));
    assert!(us.is_holiday(NaiveDate::from_ymd(2015, 12, 25))); // Christmas - Friday
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 12, 25))); // Christmas - Friday
    assert!(!us.is_holiday(NaiveDate::from_ymd(2015, 12, 26)));
    assert!(!us.is_bday(NaiveDate::from_ymd(2015, 12, 26))); // Saturday

    assert!(us.is_holiday(NaiveDate::from_ymd(2010, 12, 31))); // new years day observed
    assert!(!us.is_bday(NaiveDate::from_ymd(2010, 12, 31))); // new years day observed
    assert!(us.is_holiday(NaiveDate::from_ymd(2004, 12, 31))); // new years day observed
    assert!(!us.is_bday(NaiveDate::from_ymd(2004, 12, 31))); // new years day observed

    assert!(us.is_bday(NaiveDate::from_ymd(2013, 3, 28)));
    assert!(us.is_bday(NaiveDate::from_ymd(2013, 3, 29))); // good friday
    assert!(!us.is_bday(NaiveDate::from_ymd(2013, 3, 30)));
    assert!(!us.is_holiday(NaiveDate::from_ymd(2013, 3, 30)));
}

#[test]
fn test_us_settlement_no_cache() {
    let us = calendars::us::USSettlement;
    us_settlement_tests(us);
}

#[test]
fn test_us_settlement_cached() {
    let d0 = NaiveDate::from_ymd(1980, 1, 1);
    let d1 = NaiveDate::from_ymd(2018, 1, 1);
    let cal = HolidayCalendarCache::new(calendars::us::USSettlement, d0, d1);
    us_settlement_tests(cal);
}

#[test]
fn test_brazil_exchange() {
    let cal = calendars::brazil::BrazilExchange;

    assert!(!cal.is_bday(NaiveDate::from_ymd(2017, 11, 19))); // Sunday
    assert!(!cal.is_bday(NaiveDate::from_ymd(2017, 11, 20))); //Conciência Negra (segunda)
    assert!(cal.is_bday(NaiveDate::from_ymd(2017, 11, 21))); // Tuesday

    assert!(cal.is_bday(NaiveDate::from_ymd(2013, 5, 29))); // wednesday
    assert!(!cal.is_bday(NaiveDate::from_ymd(2013, 5, 30))); // Corpus Christi (National Holiday)
    assert!(cal.is_bday(NaiveDate::from_ymd(2013, 5, 31))); // friday

    // BrazilExchange 2019 calendar
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 01, 01)) == false ); // Confraternização Universal
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 03, 04)) == false ); // Carnaval
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 03, 05)) == false ); // Carnaval
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 04, 19)) == false ); // Paixão de Cristo
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 05, 01)) == false ); // Dia do Trabalho
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 06, 20)) == false ); // Corpus Christi
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 11, 15)) == false ); // Proclamação da República
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 12, 24)) == false ); // Véspera de Natal
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 12, 25)) == false ); // Natal
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 12, 31)) == false ); // bank holiday
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 01, 25)) == false ); // Aniversário de São Paulo
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 07, 09)) == false ); // Revolução Constitucionalista
    assert!( cal.is_bday(NaiveDate::from_ymd(2019, 11, 20)) == false ); // Dia da Consciência Negra

    // BrazilExchange 2020 calendar
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 05, 20)) == true );
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 05, 21)) == true );
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 05, 22)) == true );
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 05, 25)) == true );
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 06, 11)) == false );
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 07, 09)) == true ); // 2020 update by Ofício Circular 072/2020-PRE
    assert!( cal.is_bday(NaiveDate::from_ymd(2020, 11, 20)) == true ); // 2020 update by Ofício Circular 072/2020-PRE

    // BrazilExchange 2021 calendar
    assert!( cal.is_holiday(NaiveDate::from_ymd(2021, 1, 25)) == true );
    assert!( cal.is_holiday(NaiveDate::from_ymd(2021, 7, 9)) == true );

    // BrazilExchange 2022 calendar
    assert!( cal.is_holiday(NaiveDate::from_ymd(2022, 1, 25)) == false ); // updated by Ofício Circular 150/2020-PRE
}
