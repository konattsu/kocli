use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::File,
    io::{self, BufReader},
};

const FORECAST_URL: &str = "https://www.jma.go.jp/bosai/forecast/data/forecast/";
const FORECAST_BRIEF_URL: &str =
    "https://www.jma.go.jp/bosai/forecast/data/overview_forecast/";
const FORECAST_EXT: &str = ".json";

pub struct ForecastCode {
    code: String,
    code_mo: String, // Meteorological Observatoryをもつ codeより広域
}

#[derive(Serialize, Deserialize)]
pub struct PairTimeForecast {
    time: DateTime<FixedOffset>,
    forecast: Forecast,
}

#[derive(Serialize, Deserialize)]
enum Forecast {
    Recent(),
    Weekly(),
}

struct Recent {
    weather: String,
    wind:
}




impl ForecastCode {
    pub fn new(code: String) -> io::Result<ForecastCode> {
        Self::is_valid_area_code(&code)?;
        let code_mo = Self::generate_code_mo(&code);
        Ok(ForecastCode { code, code_mo })
    }

    fn is_valid_area_code(code: &str) -> io::Result<()> {
        const PATH: &str = "./resources/area_code.json";
        const KEY: &str = "code";

        if !Self::is_exists_in_file(PATH, code, KEY) {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid code"))
        } else {
            Ok(())
        }
    }

    fn is_exists_in_file(file_path: &str, code: &str, key: &str) -> bool {
        let file = File::open(file_path).expect("Failed to open file");
        let reader = BufReader::new(file);

        let json: serde_json::Value =
            serde_json::from_reader(reader).expect("Failed to parse json");
        let codes = json[key].as_array().expect("Invalid format");
        codes.iter().any(|c| c.as_str() == Some(code))
    }

    fn generate_code_mo(code: &str) -> String {
        format!("{}000", &code[0..=2])
    }
}

impl ForecastCode {
    pub fn three_days_forecast(&self) {
        //
    }
    pub fn week_forecast(&self) {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_area_code_valid() {
        let code = String::from("130010");
        let code_mo = String::from("130000");
        let forecast = ForecastCode::new(code);
        assert!(forecast.is_ok());
        assert_eq!(forecast.unwrap().code_mo, code_mo);
    }

    #[test]
    fn test_is_valid_area_code_invalid() {
        let code = String::from("111111");
        let forecast = ForecastCode::new(code);
        assert!(forecast.is_err());
    }
}
