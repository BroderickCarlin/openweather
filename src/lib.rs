#![forbid(unsafe_code)]

use serde;
use serde_json;

use time;
use url;

mod location;
mod parameters;
mod weather_types;

pub use location::LocationSpecifier;
pub use parameters::{Language, Settings, Unit};

use log::debug;
use url::Url;
pub use weather_types::*;

static API_BASE: &str = "https://api.openweathermap.org/data/2.5/";

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Openweather API error: {0}")]
    Api(ErrorReport),
    #[error("Error parsing to json: {0}")]
    Parsing(#[from] serde_json::Error),
    #[error("Http-Req error: {0}")]
    Connection(#[from] http_req::error::Error),
    #[error("Bad input: {msg}")]
    Input { msg: String },
    #[error("Error parsing url: {0}")]
    UrlParsing(#[from] url::ParseError),
}

/// A specialized Result type for prometheus.
pub type Result<T> = core::result::Result<T, Error>;

fn get<T>(url: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut res = Vec::new();

    let status = http_req::request::get(url, &mut res)?;
    debug!("Url: {:?}", url);
    debug!("Status: {:?}", status);
    debug!("Body_utf8: {:?}", res);
    let res = String::from_utf8_lossy(&res);
    debug!("Body_String: {:?}", res);
    // fn err_report(res: &str) -> Result<()> {
    //     let res: ErrorReport = serde_json::from_str(&res).map(?;
    //     Ok(())
    // }
    match serde_json::from_str(&res) {
        Ok(val) => Ok(val),
        Err(_) => {
            let err_report: ErrorReport = serde_json::from_str(&res)?;
            Err(Error::Api(err_report))
        }
    }
    // let res = serde_json::from_str(&res).map_err(|_| {
    //     let res: ErrorReport = serde_json::from_str(&res)?;
    //     Err(Error::Report(res))
    // })?;
    // res
    // serde.json
    // let data: T = match serde_json::from_str(&res).map_err(|e| ? {
    //     Ok(val) => val,
    //     Err(_) => {
    //         let err_report: ErrorReport = match serde_json::from_str(&res) {
    //             Ok(report) => report,
    //             Err(e) => {
    //                 return Err(ErrorReport {
    //                     cod: 0,
    //                     message: format!(
    //                         "Got unexpected response: {:?} from (parsing) error: {:?}",
    //                         res, e
    //                     ),
    //                 })?;
    //             }
    //         };
    //         return Err(err_report);
    //     }
    // };
    // Ok(data)
}

pub fn get_current_weather(
    location: &LocationSpecifier,
    key: &str,
    settings: &Settings,
) -> Result<WeatherReportCurrent> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("weather");
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_5_day_forecast(
    location: &LocationSpecifier,
    key: &str,
    settings: &Settings,
) -> Result<WeatherReport5Day> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("forecast");
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_16_day_forecast(
    location: &LocationSpecifier,
    key: &str,
    len: u8,
    settings: &Settings,
) -> Result<WeatherReport16Day> {
    if len > 16 || len == 0 {
        return Err(Error::Input {
            msg: format!("Only support 1 to 16 day forecasts but {:?} requested", len),
        });
    }
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("forecast/daily");
    params.push(("cnt".to_string(), format!("{}", len)));
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_historical_data(
    location: &LocationSpecifier,
    key: &str,
    start: time::Timespec,
    end: time::Timespec,
    settings: &Settings,
) -> Result<WeatherReportHistorical> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("history/city");
    params.push(("type".to_string(), "hour".to_string()));
    params.push(("start".to_string(), format!("{}", start.sec)));
    params.push(("end".to_string(), format!("{}", end.sec)));
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_accumulated_temperature_data(
    location: &LocationSpecifier,
    key: &str,
    start: time::Timespec,
    end: time::Timespec,
    threshold: u32,
    settings: &Settings,
) -> Result<WeatherAccumulatedTemperature> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("history/accumulated_temperature");
    params.push(("type".to_string(), "hour".to_string()));
    params.push(("start".to_string(), format!("{}", start.sec)));
    params.push(("end".to_string(), format!("{}", end.sec)));
    params.push(("threshold".to_string(), format!("{}", threshold)));
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_accumulated_precipitation_data(
    location: &LocationSpecifier,
    key: &str,
    start: time::Timespec,
    end: time::Timespec,
    threshold: u32,
    settings: &Settings,
) -> Result<WeatherAccumulatedPrecipitation> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("history/accumulated_precipitation");
    params.push(("type".to_string(), "hour".to_string()));
    params.push(("start".to_string(), format!("{}", start.sec)));
    params.push(("end".to_string(), format!("{}", end.sec)));
    params.push(("threshold".to_string(), format!("{}", threshold)));
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_current_uv_index(
    location: &LocationSpecifier,
    key: &str,
    settings: &Settings,
) -> Result<UvIndex> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("uvi");
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_forecast_uv_index(
    location: &LocationSpecifier,
    key: &str,
    len: u8,
    settings: &Settings,
) -> Result<ForecastUvIndex> {
    if len > 8 || len == 0 {
        return Err(Error::Input {
            msg: format!("Only support 1 to 8 day forecasts but {:?} requested", len),
        });
    }
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("uvi/forecast");
    params.push(("cnt".to_string(), format!("{}", len)));
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

pub fn get_historical_uv_index(
    location: &LocationSpecifier,
    key: &str,
    start: time::Timespec,
    end: time::Timespec,
    settings: &Settings,
) -> Result<HistoricalUvIndex> {
    let mut base = String::from(API_BASE);
    let mut params = location.format();

    base.push_str("uvi/history");
    params.push(("start".to_string(), format!("{}", start.sec)));
    params.push(("end".to_string(), format!("{}", end.sec)));
    params.push(("APPID".to_string(), key.to_string()));
    params.append(&mut settings.format());

    let url = Url::parse_with_params(&base, params)?;
    get(&url.as_str())
}

#[cfg(test)]
mod tests {
    use crate::{LocationSpecifier, Settings};
    static SETTINGS: &Settings = &Settings {
        unit: None,
        lang: None,
    };

    use dotenv;
    fn api_key() -> String {
        let key = "API_KEY";
        dotenv::var(key).expect("get api key for testing from .env file")
    }

    #[test]
    fn get_current_weather() {
        let loc = LocationSpecifier::CityAndCountryName {
            city: "Minneapolis".into(),
            country: "USA".into(),
        };
        let weather = crate::get_current_weather(&loc, &api_key(), SETTINGS)
            .expect("failure getting current weather");
        println!("Right now in Minneapolis, MN it is {}C", weather.main.temp);
    }

    #[test]
    fn get_5_day_forecast() {
        let loc = LocationSpecifier::CityAndCountryName {
            city: "Minneapolis".into(),
            country: "USA".into(),
        };
        let weather = crate::get_5_day_forecast(&loc, &api_key(), SETTINGS)
            .expect("failure getting 5 day forecast");
        println!("5 Day Report in Minneapolis, MN it is {:?}", weather.list);
    }
}
