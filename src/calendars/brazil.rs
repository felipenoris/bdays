use crate::easter;
use crate::HolidayCalendar;
use ::chrono::{Datelike, Weekday};

/// Brazilian banking holidays.
/// This calendar is defined by brazilian federal holidays plus Carnival.
pub struct BRSettlement;

fn is_brazilian_national_holiday<T: Datelike + Copy + PartialOrd>(date: T) -> bool {
    let (yy, mm, dd) = (date.year(), date.month(), date.day());

    // Bisection
    if mm >= 8 {
        // Fixed holidays
        if
        // Independencia do Brasil
        ((mm == 9) && (dd == 7))
            ||
            // Nossa Senhora Aparecida
            ((mm == 10) && (dd == 12))
            ||
            // Finados
            ((mm == 11) && (dd == 2))
            ||
            // Proclamacao da Republica
            ((mm == 11) && (dd == 15))
            ||
            // Natal
            ((mm == 12) && (dd == 25))
        {
            return true;
        }
    } else {
        // mm < 8
        // Fixed holidays
        if
        // Confraternizacao Universal
        ((mm == 1) && (dd == 1))
            ||
            // Tiradentes
            ((mm == 4) && (dd == 21))
            ||
            // Dia do Trabalho
            ((mm == 5) && (dd == 1))
        {
            return true;
        }

        // Easter occurs up to April, so Corpus Christi will be up to July in the worst case, which is before August (mm < 8).
        // Holidays based on easter date.
        let dt_rata = date.num_days_from_ce();
        let e_rata = easter::easter_num_days_from_ce(yy).unwrap();

        if
        // Segunda de Carnaval
        ( dt_rata == ( e_rata - 48 ) )
            ||
            // Terca de Carnaval
            ( dt_rata == ( e_rata - 47 ) )
            ||
            // Sexta-feira Santa
            ( dt_rata == ( e_rata -  2 ) )
            ||
            // Corpus Christi
            ( dt_rata == ( e_rata + 60 ) )
        {
            return true;
        }
    }

    false
}

impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for BRSettlement {
    fn is_holiday(&self, date: T) -> bool {
        is_brazilian_national_holiday(date)
    }
}

/// B3 Exchange holidays (<https://www.b3.com.br>).
pub struct BrazilExchange;

impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for BrazilExchange {
    fn is_holiday(&self, date: T) -> bool {
        let (yy, mm, dd) = (date.year(), date.month(), date.day());

        if
        // Aniversário de São Paulo
        ( mm == 1 && dd == 25 && yy < 2022 )
            ||
            // Revolucão
            ( mm == 7 && dd == 9 && yy != 2020 && yy < 2022 )
            ||
            // Consciência Negra (since 2007)
            ( yy >= 2007 && mm == 11 && dd == 20 && yy != 2020 && yy < 2022 )
            // Christmas Eve
            ||
            ( mm == 12 && dd == 24)
            ||
            // Último dia útil do ano
            ( mm == 12 && (dd == 31 || (dd>=29 && date.weekday() == Weekday::Fri) ))
        {
            return true;
        }

        // national holidays
        is_brazilian_national_holiday(date)
    }
}
