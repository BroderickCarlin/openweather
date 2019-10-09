extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate time;
extern crate url;

mod weather_types;

pub use weather_types::*;
use url::Url;

static API_BASE: &str = "https://api.openweathermap.org/data/2.5/";

#[derive(Debug)]
pub enum LocationSpecifier<'a>{
    CityAndCountryName {city: &'a str, country: &'a str},
    CityId(&'a str),
    Coordinates {lat: f32, lon: f32},
    ZipCode {zip: &'a str, country: &'a str},

    // The following location specifiers are used to specify multiple cities or a region
    BoundingBox {lon_left: f32, lat_bottom: f32, lon_right: f32, lat_top: f32, zoom: f32},
    Circle {lat: f32, lon: f32, count: u16},
    CityIds(Vec<&'a str>),
}

#[derive(Debug)]
pub enum Unit {
    // Celcius, default
    Metric,
    // Kelvin
    Standard,
    // Fahrenheit
    Imperial,
}

impl Unit {
    pub fn format(&self) -> Option<(String, String)> {
        match self {
            Unit::Metric => Some(("units".to_string(), "metric".to_string())),
            Unit::Standard => None,
            Unit::Imperial => Some(("units".to_string(), "imperial".to_string())),
        }
    }
}


impl<'a> LocationSpecifier<'a> {
    pub fn format(&'a self) -> Vec<(String, String)> {
        match &self {
            LocationSpecifier::CityAndCountryName {city, country} => {
                if *country == "" {
                    return vec![("q".to_string(), city.to_string())];
                } else {
                    return vec![("q".to_string(), format!("{},{}", city, country))];
                }
            }
            LocationSpecifier::CityId(id) => {
                return vec![("id".to_string(), id.to_string())];
            }
            LocationSpecifier::Coordinates {lat, lon} => {
                return vec![("lat".to_string(), format!("{}",lat)), 
                            ("lon".to_string(), format!("{}",lon))];
            }
            LocationSpecifier::ZipCode {zip, country} => {
                if *country == "" {
                    return vec![("zip".to_string(), zip.to_string())];
                } else {
                    return vec![("zip".to_string(), format!("{},{}", zip, country))];
                }
            }
            LocationSpecifier::BoundingBox {lon_left, lat_bottom, lon_right, lat_top, zoom} => {
                return vec![("bbox".to_string(), format!("{},{},{},{},{}", lon_left, lat_bottom, lon_right, lat_top, zoom))];
            }
            LocationSpecifier::Circle {lat, lon, count} => {
                return vec![("lat".to_string(), format!("{}", lat)), 
                            ("lon".to_string(), format!("{}", lon)), 
                            ("cnt".to_string(), format!("{}", count))];
            }
            LocationSpecifier::CityIds(ids) => {
                let mut locations: String = "".to_string();
                for loc in ids { locations += loc; }
                return vec![("id".to_string(), locations.to_string())];
            }
        }
    }
}

fn get<T>(url: &str) -> Result<T, ErrorReport> where T: serde::de::DeserializeOwned {
    let res = reqwest::get(url).unwrap().text().unwrap();
    let data: T = match serde_json::from_str(&res) {
        Ok(val) => {val},
        Err(_) => {
            let err_report: ErrorReport = match serde_json::from_str(res.as_str()) {
                Ok(report) => {report},
                Err(_) => {
                    return Err(ErrorReport{cod: 0, message: format!("Got unexpected response: {:?}", res)});
                }
            };
            return Err(err_report);
        }
    };
    Ok(data)
}

pub fn get_current_weather(location: LocationSpecifier, key: &str) -> Result<WeatherReportCurrent, ErrorReport> {
	let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("weather");
	params.push(("APPID".to_string(), key.to_string()));
    params.push(("units".to_string(), "metric".to_string()));
    params.push(("lang".to_string(), "de".to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_5_day_forecast(location: LocationSpecifier, key: &str) -> Result<WeatherReport5Day, ErrorReport> {
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("forecast");
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_16_day_forecast(location: LocationSpecifier, key: &str, len: u8) -> Result<WeatherReport16Day, ErrorReport> {
    if len > 16 || len == 0 {
        return Err(ErrorReport{cod: 0, message: format!("Only support 1 to 16 day forecasts but {:?} requested", len)});
    }
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("forecast/daily");
	params.push(("cnt".to_string(), format!("{}", len)));
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_historical_data(location: LocationSpecifier, key: &str, start: time::Timespec, end: time::Timespec) -> Result<WeatherReportHistorical, ErrorReport> {
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("history/city");
	params.push(("type".to_string(), "hour".to_string()));
	params.push(("start".to_string(), format!("{}", start.sec)));
	params.push(("end".to_string(), format!("{}", end.sec)));
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_accumulated_temperature_data(location: LocationSpecifier, key: &str, start: time::Timespec, end: time::Timespec, threshold: u32) -> Result<WeatherAccumulatedTemperature, ErrorReport> {
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("history/accumulated_temperature");
	params.push(("type".to_string(), "hour".to_string()));
	params.push(("start".to_string(), format!("{}", start.sec)));
	params.push(("end".to_string(), format!("{}", end.sec)));
	params.push(("threshold".to_string(), format!("{}", threshold)));
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_accumulated_precipitation_data(location: LocationSpecifier, key: &str, start: time::Timespec, end: time::Timespec, threshold: u32) -> Result<WeatherAccumulatedPrecipitation, ErrorReport> {
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("history/accumulated_precipitation");
	params.push(("type".to_string(), "hour".to_string()));
	params.push(("start".to_string(), format!("{}", start.sec)));
	params.push(("end".to_string(), format!("{}", end.sec)));
	params.push(("threshold".to_string(), format!("{}", threshold)));
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_current_uv_index(location: LocationSpecifier, key: &str) -> Result<UvIndex, ErrorReport> {
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("uvi");
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_forecast_uv_index(location: LocationSpecifier, key: &str, len: u8) -> Result<ForecastUvIndex, ErrorReport> {
    if len > 8 || len == 0 {
        return Err(ErrorReport{cod: 0, message: format!("Only support 1 to 8 day forecasts but {:?} requested", len)});
    }
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("uvi/forecast");
	params.push(("cnt".to_string(), format!("{}", len)));
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}

pub fn get_historical_uv_index(location: LocationSpecifier, key: &str, start: time::Timespec, end: time::Timespec) -> Result<HistoricalUvIndex, ErrorReport> {
    let mut base = String::from(API_BASE);
	let mut params = location.format();

	base.push_str("uvi/history");
	params.push(("start".to_string(), format!("{}", start.sec)));
	params.push(("end".to_string(), format!("{}", end.sec)));
	params.push(("APPID".to_string(), key.to_string()));

	let url = Url::parse_with_params(&base, params).unwrap();
    get(&url.as_str())
}
