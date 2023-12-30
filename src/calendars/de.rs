use crate::HolidayCalendar;
use ::chrono::Datelike;

fn check_newyear<T: Datelike>(date: &T) -> bool {
    date.day() == 1 && date.month() == 1
}

fn check_epiphany<T: Datelike>(date: &T) -> bool {
    date.day() == 6 && date.month() == 1
}

fn check_intwomansday<T: Datelike>(date: &T) -> bool {
    date.day() == 8 && date.month() == 3
}

fn check_goodfriday<T: Datelike, T2: Datelike>(date: &T, easter_sunday: &T2) -> bool {
    date.num_days_from_ce() == easter_sunday.num_days_from_ce() - 2
}

fn check_eastermonday<T: Datelike, T2: Datelike>(date: &T, easter_sunday: &T2) -> bool {
    date.num_days_from_ce() == easter_sunday.num_days_from_ce() + 1
}

fn check_labourday<T: Datelike>(date: &T) -> bool {
    date.day() == 1 && date.month() == 5
}

fn check_ascensionday<T: Datelike, T2: Datelike>(date: &T, easter_sunday: &T2) -> bool {
    date.num_days_from_ce() == easter_sunday.num_days_from_ce() + 39
}

fn check_whitmonday<T: Datelike, T2: Datelike>(date: &T, easter_sunday: &T2) -> bool {
    date.num_days_from_ce() == easter_sunday.num_days_from_ce() + 50
}

fn check_corpuschristi<T: Datelike, T2: Datelike>(date: &T, easter_sunday: &T2) -> bool {
    date.num_days_from_ce() == easter_sunday.num_days_from_ce() + 60
}

fn check_assumptionday<T: Datelike>(date: &T) -> bool {
    date.day() == 15 && date.month() == 8
}

fn check_childrenday<T: Datelike>(date: &T) -> bool {
    date.day() == 20 && date.month() == 9
}

fn check_unityday<T: Datelike>(date: &T) -> bool {
    date.day() == 3 && date.month() == 10
}

fn check_reformationday<T: Datelike>(date: &T) -> bool {
    date.day() == 31 && date.month() == 10
}

fn check_allsaintsday<T: Datelike>(date: &T) -> bool {
    date.day() == 1 && date.month() == 11
}

fn check_repentence<T: Datelike>(date: &T) -> bool {
    let november_22 = chrono::NaiveDate::from_ymd_opt(date.year(), 11, 22)
        .expect("22 Nov should exist every year");
    november_22
        .iter_days()
        .rev()
        .take(7)
        .find(|d| d.weekday() == chrono::Weekday::Wed)
        .expect("One of the last 7 days must be a Wednesday")
        .num_days_from_ce()
        == date.num_days_from_ce()
}

fn check_christmasday<T: Datelike>(date: &T) -> bool {
    date.day() == 25 && date.month() == 12
}

fn check_2ndchristmasday<T: Datelike>(date: &T) -> bool {
    date.day() == 26 && date.month() == 12
}

/// Enum lising all possible holidays
#[derive(Clone, Copy, Debug)]
enum GermanStateHoliday {
    /// Neujahrstag
    NewYearsDay,
    /// Heilige drei Könige
    Epiphany,
    /// Internationaler Frauentag
    InternationalWomensDay,
    /// Karfreitag
    GoodFriday,
    /// Ostermontag
    EasterMonday,
    /// Tag der Arbeit
    LabourDay,
    /// Christi Himmelfahrt
    AscensionDay,
    /// Pfingstmontag
    WhitMonday,
    /// Fronleichnam
    CorpusChristi,
    /// Mariä Himmelfahrt
    AssumptionDay,
    /// Weltkindertag
    WorldChildrensDay,
    /// Tag der Deutschen Einheit
    GermanUnityDay,
    /// Reformationstag
    ReformationDay,
    /// Allerheiligen
    AllSaintsDay,
    /// Buß- und Bettag
    RepentanceAndPrayerDay,
    /// Erster Weihnachtstag
    ChristmasDay,
    /// Zweiter Weihnachtstag
    SecondDayOfChristmas,
}

impl GermanStateHoliday {
    fn from_date<T: Datelike>(date: T) -> Option<Self> {
        let easter_sunday = crate::easter::easter_naive_date(date.year()).ok()?;

        if check_newyear(&date) {
            Some(Self::NewYearsDay)
        } else if check_epiphany(&date) {
            Some(Self::Epiphany)
        } else if check_intwomansday(&date) {
            Some(Self::InternationalWomensDay)
        } else if check_goodfriday(&date, &easter_sunday) {
            Some(Self::GoodFriday)
        } else if check_eastermonday(&date, &easter_sunday) {
            Some(Self::EasterMonday)
        } else if check_labourday(&date) {
            Some(Self::LabourDay)
        } else if check_ascensionday(&date, &easter_sunday) {
            Some(Self::AscensionDay)
        } else if check_whitmonday(&date, &easter_sunday) {
            Some(Self::WhitMonday)
        } else if check_corpuschristi(&date, &easter_sunday) {
            Some(Self::CorpusChristi)
        } else if check_assumptionday(&date) {
            Some(Self::AssumptionDay)
        } else if check_childrenday(&date) {
            Some(Self::WorldChildrensDay)
        } else if check_unityday(&date) {
            Some(Self::GermanUnityDay)
        } else if check_reformationday(&date) {
            Some(Self::ReformationDay)
        } else if check_allsaintsday(&date) {
            Some(Self::AllSaintsDay)
        } else if check_repentence(&date) {
            Some(Self::RepentanceAndPrayerDay)
        } else if check_christmasday(&date) {
            Some(Self::ChristmasDay)
        } else if check_2ndchristmasday(&date) {
            Some(Self::SecondDayOfChristmas)
        } else {
            None
        }
    }

    fn holidays() -> [GermanStateHoliday; 17] {
        use GermanStateHoliday::*;
        [
            NewYearsDay,            // Neujahrstag
            Epiphany,               // Heilige drei Könige
            InternationalWomensDay, // Internationaler Frauentag
            GoodFriday,             // Karfreitag
            EasterMonday,           // Ostermontag
            LabourDay,              // Tag der Arbeit
            AscensionDay,           // Christi Himmelfahrt
            WhitMonday,             // Pfingstmontag
            CorpusChristi,          // Fronleichnam
            AssumptionDay,          // Mariä Himmelfahrt
            WorldChildrensDay,      // Weltkindertag
            GermanUnityDay,         // Tag der Deutschen Einheit
            ReformationDay,         // Reformationstag
            AllSaintsDay,           // Allerheiligen
            RepentanceAndPrayerDay, // Buß- und Bettag
            ChristmasDay,           // Erster Weihnachtstag
            SecondDayOfChristmas,   // Zweiter Weihnachtstag
        ]
    }
}

/// Enum for all German states, plus a variant for holidays effective in any state.
/// These implement [HolidayCalendar](crate::HolidayCalendar)
#[derive(Clone, Copy, Debug)]
pub enum GermanState {
    /// Baden-Württemberg
    BW,
    /// Bayern
    BY,
    /// Berlin
    BE,
    /// Brandenburg
    BB,
    /// Bremen
    HB,
    /// Hamburg
    HH,
    /// Hessen
    HE,
    /// Mecklenburg-Vorpommern
    MV,
    /// Niedersachsen
    NI,
    /// Nordrhein-Westfalen
    NW,
    /// Rheinland-Pfalz
    RP,
    /// Saarland
    SL,
    /// Sachsen
    SN,
    /// Sachsen-Anhalt
    ST,
    /// Schleswig-Holstein
    SH,
    /// Thüringen
    TH,
    /// Checks for holidays in any state
    ANY,
}

impl GermanState {
    /// Returns the number of holidays in the given state and year
    ///
    /// # Arguments
    ///
    /// * `year` - The year for which to count the number of holidays
    ///
    /// # Example
    ///
    /// ```
    /// use bdays::calendars::de::GermanState;
    /// assert_eq!(GermanState::NI.num_holidays(2023), 10);
    /// ```
    pub fn num_holidays(&self, year: i32) -> u32 {
        GermanStateHoliday::holidays()
            .into_iter()
            .map(|holiday| self.has_holiday(holiday, year) as u32)
            .sum()
    }

    const fn has_holiday(&self, holiday: GermanStateHoliday, year: i32) -> bool {
        match (self, holiday) {
            _ if year < 1990 => false, // Holidays pre reunification are not supported
            (_, GermanStateHoliday::NewYearsDay) => true,
            (GermanState::BW | GermanState::BY | GermanState::ST, GermanStateHoliday::Epiphany) => {
                true
            }
            (GermanState::BE, GermanStateHoliday::InternationalWomensDay) if year >= 2019 => true, // Berlin adopted International Women's Day in 2019
            (GermanState::MV, GermanStateHoliday::InternationalWomensDay) if year >= 2023 => true, // MV adopted International Women's Day in 2023
            (
                _,
                GermanStateHoliday::GoodFriday
                | GermanStateHoliday::EasterMonday
                | GermanStateHoliday::LabourDay
                | GermanStateHoliday::AscensionDay
                | GermanStateHoliday::WhitMonday,
            ) => true,
            (
                GermanState::BW
                | GermanState::BY
                | GermanState::HE
                | GermanState::NW
                | GermanState::RP
                | GermanState::SL,
                GermanStateHoliday::CorpusChristi,
            ) => true,
            (GermanState::BY | GermanState::SL, GermanStateHoliday::AssumptionDay) => true,
            (GermanState::TH, GermanStateHoliday::WorldChildrensDay) if year >= 2019 => true,
            (_, GermanStateHoliday::GermanUnityDay) => true,
            (_, GermanStateHoliday::ReformationDay) if year == 2017 => true, // Reformation Day was a one-time holiday in all states in 2017
            (
                GermanState::BB
                | GermanState::MV
                | GermanState::SN
                | GermanState::ST
                | GermanState::TH,
                GermanStateHoliday::ReformationDay,
            ) => true, // 5 states have had Reformation Day since reunification
            (
                GermanState::HB | GermanState::HH | GermanState::NI | GermanState::SH,
                GermanStateHoliday::ReformationDay,
            ) if year >= 2018 => true, // 4 states permanently adopted Reformation Day in 2018
            (
                GermanState::BW
                | GermanState::BY
                | GermanState::NW
                | GermanState::RP
                | GermanState::SL,
                GermanStateHoliday::AllSaintsDay,
            ) => true,
            (_, GermanStateHoliday::RepentanceAndPrayerDay) if year <= 1994 => true, // In 1994, Repentence and Prayer Day was discontinued (beginning in 1995) in all states except Saxony
            (GermanState::SN, GermanStateHoliday::RepentanceAndPrayerDay) => true,
            (_, GermanStateHoliday::ChristmasDay | GermanStateHoliday::SecondDayOfChristmas) => {
                true
            }
            (
                GermanState::ANY,
                GermanStateHoliday::InternationalWomensDay | GermanStateHoliday::WorldChildrensDay,
            ) if year >= 2019 => true, // First International Women's Day and International Children's Day in Germany were in 2019 (BE and TH respectively)
            (
                GermanState::ANY,
                GermanStateHoliday::Epiphany
                | GermanStateHoliday::CorpusChristi
                | GermanStateHoliday::AssumptionDay
                | GermanStateHoliday::ReformationDay
                | GermanStateHoliday::AllSaintsDay
                | GermanStateHoliday::RepentanceAndPrayerDay,
            ) => true, //Remaining days that are only holidays in some (any) states
            _ => false,
        }
    }
}

#[test]
fn test_number_of_holidays() {
    let tests = [
        (GermanState::BW, 12),
        (GermanState::BY, 13),
        (GermanState::BE, 10),
        (GermanState::BB, 10),
        (GermanState::HB, 10),
        (GermanState::HH, 10),
        (GermanState::HE, 10),
        (GermanState::MV, 11),
        (GermanState::NI, 10),
        (GermanState::NW, 11),
        (GermanState::RP, 11),
        (GermanState::SL, 12),
        (GermanState::SN, 11),
        (GermanState::ST, 11),
        (GermanState::SH, 10),
        (GermanState::TH, 11),
    ];
    for (state, num) in tests {
        assert_eq!(state.num_holidays(2023), num);
    }
}

/// This implementation doesn't return any holidays before 1990.
///
/// Before that the current Germany was separated into the "German
/// Democratic Republic" and the "Federal Republic of Germany" which both had
/// somewhat different holidays. Since this class is called "Germany" it
/// doesn't really make sense to include the days from the two former
/// countries.
///
/// Note that Germany doesn't have rules for holidays that happen on a
/// Sunday. Those holidays are still holiday days but there is no additional
/// day to make up for the "lost" day.
///
/// Also note that German holidays are partly declared by each state there
/// are some weired edge cases:
///
/// - Assumption Day / Mariä Himmelfahrt is only a holiday in Bavaria (BY) if your
///   municipality is mostly catholic which in term depends on census data.
///   Since we don't have this data but most municipalities in Bavaria
///   *are* mostly catholic, we count that as holiday for whole Bavaria.
/// - There is an Augsburg Peace Festival / Augsburger Friedensfest which only
///   exists in the Bavarian town Augsburg. This is excluded for Bavaria.
/// - Maundy Thursday / Gründonnerstag (Thursday before easter) is not a holiday
///   but pupils in Baden Württemberg (BW) don't have to go to school. It is
///   excluded from our list.
/// - Corpus Christi / Fronleichnam is a holiday in certain, explicitly defined
///   municipalities in Saxony (SN) and Thuringia (TH). We exclude it from
///   both provinces.
impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for GermanState {
    fn is_holiday(&self, date: T) -> bool {
        if let Some(holiday) = GermanStateHoliday::from_date(date) {
            self.has_holiday(holiday, date.year())
        } else {
            false
        }
    }
}

/// This implementation considers a date a holiday if it is a holiday in any
/// State in the Vec. See the implementation for [GermanState](GermanState) for details.
impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for Vec<GermanState> {
    fn is_holiday(&self, date: T) -> bool {
        if let Some(holiday) = GermanStateHoliday::from_date(date) {
            self.iter()
                .any(|state| state.has_holiday(holiday, date.year()))
        } else {
            false
        }
    }
}

#[test]
fn test_is_holiday() {
    use ::chrono::NaiveDate;
    // New Year's Day
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 1, 1).expect("Valid date")),
        true
    );

    // Epiphany
    assert_eq!(
        GermanState::BW.is_holiday(NaiveDate::from_ymd_opt(2023, 1, 6).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::BY.is_holiday(NaiveDate::from_ymd_opt(2023, 1, 6).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::ST.is_holiday(NaiveDate::from_ymd_opt(2023, 1, 6).expect("Valid date")),
        true
    );

    // International Women's Day
    assert_eq!(
        GermanState::BE.is_holiday(NaiveDate::from_ymd_opt(2020, 3, 8).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::MV.is_holiday(NaiveDate::from_ymd_opt(2023, 3, 8).expect("Valid date")),
        true
    );

    // Good Friday
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 4, 7).expect("Valid date")),
        true
    );

    // Easter Monday
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 4, 10).expect("Valid date")),
        true
    );

    // Labour Day
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 5, 1).expect("Valid date")),
        true
    );

    // Ascension Day
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 5, 18).expect("Valid date")),
        true
    );

    // Whit Monday
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 5, 29).expect("Valid date")),
        true
    );

    // Corpus Christi
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 6, 8).expect("Valid date")),
        true
    );

    // Assumption Day
    assert_eq!(
        GermanState::BY.is_holiday(NaiveDate::from_ymd_opt(2023, 8, 15).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::SL.is_holiday(NaiveDate::from_ymd_opt(2023, 8, 15).expect("Valid date")),
        true
    );

    // World's Children Day
    assert_eq!(
        GermanState::TH.is_holiday(NaiveDate::from_ymd_opt(2023, 9, 20).expect("Valid date")),
        true
    );

    // German Unity Day
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 10, 3).expect("Valid date")),
        true
    );

    // Reformation Day
    assert_eq!(
        GermanState::TH.is_holiday(NaiveDate::from_ymd_opt(2023, 10, 31).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::HB.is_holiday(NaiveDate::from_ymd_opt(2023, 10, 31).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::BY.is_holiday(NaiveDate::from_ymd_opt(2017, 10, 31).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::BB.is_holiday(NaiveDate::from_ymd_opt(1998, 10, 31).expect("Valid date")),
        true
    );

    // All Saint's Day
    assert_eq!(
        GermanState::BW.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 1).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::BY.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 1).expect("Valid date")),
        true
    );

    // Repentance and Prayer Day
    assert_eq!(
        GermanState::SN.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 22).expect("Valid date")),
        true
    );
    assert_eq!(
        GermanState::NI.is_holiday(NaiveDate::from_ymd_opt(1993, 11, 17).expect("Valid date")),
        true
    );

    // Christmas Day
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 12, 25).expect("Valid date")),
        true
    );

    // Second Christmas Day
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 12, 26).expect("Valid date")),
        true
    );

    // Non-Holidays
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 2, 1).expect("Valid date")),
        false
    ); // Non-holiday date
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 7, 15).expect("Valid date")),
        false
    ); // Non-holiday date
    assert_eq!(
        GermanState::ANY.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 20).expect("Valid date")),
        false
    ); // Non-holiday date

    // Epiphany in states where it's not observed
    assert_eq!(
        GermanState::NI.is_holiday(NaiveDate::from_ymd_opt(2023, 1, 6).expect("Valid date")),
        false
    );
    assert_eq!(
        GermanState::RP.is_holiday(NaiveDate::from_ymd_opt(2023, 1, 6).expect("Valid date")),
        false
    );

    // International Women's Day in states where it's not observed
    assert_eq!(
        GermanState::TH.is_holiday(NaiveDate::from_ymd_opt(2023, 3, 8).expect("Valid date")),
        false
    );
    assert_eq!(
        GermanState::NW.is_holiday(NaiveDate::from_ymd_opt(2023, 3, 8).expect("Valid date")),
        false
    );

    // Reformation Day in BY
    assert_eq!(
        GermanState::BY.is_holiday(NaiveDate::from_ymd_opt(2023, 10, 31).expect("Valid date")),
        false
    );

    // Repentance and Prayer Day
    assert_eq!(
        GermanState::NI.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 22).expect("Valid date")),
        false
    );

    // All Saint's Day in states where it's not observed
    assert_eq!(
        GermanState::HH.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 1).expect("Valid date")),
        false
    );
    assert_eq!(
        GermanState::NI.is_holiday(NaiveDate::from_ymd_opt(2023, 11, 1).expect("Valid date")),
        false
    );

    // Holidays in other states
    assert_eq!(
        GermanState::BY.is_holiday(NaiveDate::from_ymd_opt(2023, 10, 31).expect("Valid date")),
        false
    ); // Reformation Day in BY
    assert_eq!(
        GermanState::NI.is_holiday(NaiveDate::from_ymd_opt(2023, 8, 15).expect("Valid date")),
        false
    ); // Assumption Day in NI
    assert_eq!(
        GermanState::SL.is_holiday(NaiveDate::from_ymd_opt(2023, 9, 20).expect("Valid date")),
        false
    ); // World's Children Day in SL
    assert_eq!(
        GermanState::BW.is_holiday(NaiveDate::from_ymd_opt(2023, 3, 8).expect("Valid date")),
        false
    ); // International Women's Day in BW
}
