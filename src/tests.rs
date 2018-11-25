
use super::{WeekendsOnly, HolidayCalendar, easter, brazil};
use chrono::{NaiveDate, Datelike};

#[test]
fn test_weekend_calendar() {

    let cal = WeekendsOnly;

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

#[test]
fn test_br_settlement() {
    // Brazil HolidayCalendar tests
    let cal = brazil::BRSettlement;

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
