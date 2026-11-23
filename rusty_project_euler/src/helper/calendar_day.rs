use std::cmp::Ordering;
use std::fmt::Display;

pub struct CalendarDay {
    pub weekday: u32,
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl CalendarDay {
    pub fn new(weekday: u32, day: u32, month: u32, year: u32) -> Self {
        CalendarDay {
            weekday,
            day,
            month,
            year,
        }
    }

    pub fn next(&self) -> CalendarDay {
        let next_weekday = (self.weekday + 1) % 7;
        let mut next_day = self.day + 1;
        let mut next_month = self.month;
        let mut next_year = self.year;

        let days_in_month = days_in_month(self.month, self.year);

        if next_day > days_in_month {
            next_day = 1;
            next_month += 1;
        }

        if next_month > 12 {
            next_month = 1;
            next_year += 1;
        }

        CalendarDay::new(next_weekday, next_day, next_month, next_year)
    }

    pub fn prev(&self) -> CalendarDay {
        let prev_weekday = (self.weekday - 1) % 7; // Go back one day in the week
        let mut prev_day = self.day - 1;
        let mut prev_month = self.month;
        let mut prev_year = self.year;

        if prev_day == 0 {
            prev_month -= 1;
            if prev_month == 0 {
                prev_month = 12;
                prev_year -= 1;
            }
            prev_day = days_in_month(prev_month, prev_year);
        }

        CalendarDay::new(prev_weekday, prev_day, prev_month, prev_year)
    }
}

fn days_in_month(month: u32, year: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

impl Display for CalendarDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}, {}",
            weekday_text(self.weekday),
            month_text(self.month),
            self.day,
            self.year
        )
    }
}

fn weekday_text(weekday: u32) -> &'static str {
    match weekday {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Unknown",
    }
}

fn month_text(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

impl Ord for CalendarDay {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
    }
}

impl PartialOrd for CalendarDay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CalendarDay {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl Eq for CalendarDay {}
