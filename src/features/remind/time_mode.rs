use std::io;

use chrono::{Local, NaiveDateTime, NaiveTime, TimeDelta, Timelike};
use clap::ValueEnum;

#[derive(ValueEnum, Clone, PartialEq)]
pub enum TimeMode {
    Time,  // A specific time
    Later, // How long from now, e.g. after 2 minutes
}
impl TimeMode {
    const ALLOWED_TIME_FMT: &'static str = "%H:%M:%S";

    pub fn calc_time(&self, input: &str) -> io::Result<u64> {
        let nt = Self::parse_str_to_naivetime(input)?;
        match *self {
            Self::Later => Ok(nt.num_seconds_from_midnight() as u64),
            Self::Time => Ok(Self::diff_current_time(&nt)),
        }
    }

    fn parse_str_to_naivetime(input: &str) -> io::Result<NaiveTime> {
        match NaiveTime::parse_from_str(input, Self::ALLOWED_TIME_FMT) {
            Ok(nt) => Ok(nt),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, e.to_string())),
        }
    }

    fn diff_current_time(nt: &NaiveTime) -> u64 {
        let today = Local::now().date_naive();
        let input_ndt = NaiveDateTime::new(today, *nt);
        let now = Local::now().naive_local();
        let diff = input_ndt - now;
        if diff.num_seconds() > 0 {
            return diff.num_seconds().to_string().parse::<u64>().unwrap();
        }
        // When the time of the day following the current time is entered
        // Often happens late at night
        let diff = (input_ndt + TimeDelta::try_days(1).unwrap()) - now;
        if diff.num_seconds() > 0 {
            return diff.num_seconds().to_string().parse::<u64>().unwrap();
        }
        panic!("Unexpected error: diff: {}", diff);
    }
}
