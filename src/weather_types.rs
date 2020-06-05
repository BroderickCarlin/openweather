use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CityShort {
    pub id: u32,
    pub name: String,
    pub coord: Coordinates,
    pub country: String,
    pub population: u32,
    pub timezone: i32,
    pub sunrise: u64,
    pub sunset: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CityLong {
    pub geoname_id: u64,
    pub name: String,
    pub lat: u32,
    pub lon: u32,
    pub country: String,
    pub iso2: String,
    #[serde(rename = "type")]
    pub city_type: String,
    pub population: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Main {
    pub temp: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    #[serde(default)]
    pub sea_level: Option<f32>,
    #[serde(default)]
    pub grnd_level: Option<f32>,
    pub humidity: f32,
    #[serde(default)]
    pub temp_kf: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Wind {
    pub speed: f32,
    pub deg: Option<f32>,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Rain {
    /// Rain volume in mm
    #[serde(rename = "3h")]
    pub three_h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Snow {
    /// Snow volume
    #[serde(rename = "3h")]
    pub three_h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct System {
    pub pod: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct TempDaily {
    pub day: f32,
    pub min: f32,
    pub max: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct TimeSliceHourly {
    pub dt: u64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub sys: System,
    pub dt_txt: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct TimeSliceDaily {
    pub dt: u64,
    pub temp: TempDaily,
    pub pressure: f32,
    pub humidity: f32,
    pub weather: Vec<Weather>,
    pub speed: f32,
    pub deg: u32,
    pub clouds: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ErrorReport {
    pub cod: u32,
    pub message: String,
}

use core::fmt;
impl fmt::Display for ErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code: {}, Message: {}", self.cod, self.message)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Sys {
    #[serde(rename = "type")]
    pub message_type: u32,
    pub id: u32,
    pub message: Option<f32>,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherReportCurrent {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub timezone: Option<i32>,
    pub id: u64,
    pub name: String,
    pub message: Option<String>,
    pub cod: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherReport5Day {
    pub cod: String,
    pub message: Option<f32>,
    pub cnt: u8,
    pub list: Vec<TimeSliceHourly>,
    pub city: CityShort,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherReport16Day {
    pub cod: String,
    pub message: f32,
    pub city: CityLong,
    pub cnt: u8,
    pub list: Vec<TimeSliceDaily>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherReportHistoricalElement {
    pub main: Main,
    pub wind: Wind,
    pub clouds: Clouds,
    pub weather: Vec<Weather>,
    pub dt: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherReportHistorical {
    pub message: String,
    pub cod: String,
    pub city_id: u64,
    pub calctime: f32,
    pub cnt: u32,
    pub list: Vec<WeatherReportHistoricalElement>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherAccumulatedTemperatureElement {
    pub date: String,
    pub temp: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherAccumulatedPrecipitation {
    pub message: String,
    pub cod: String,
    pub city_id: u64,
    pub calctime: u64,
    pub list: Vec<WeatherAccumulatedPrecipitationElement>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherAccumulatedPrecipitationElement {
    pub date: String,
    pub rain: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeatherAccumulatedTemperature {
    pub message: String,
    pub cod: String,
    pub city_id: u64,
    pub calctime: u64,
    pub list: Vec<WeatherAccumulatedTemperatureElement>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct UvIndex {
    pub lat: f32,
    pub lon: f32,
    pub data_iso: String,
    pub date: u64,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ForecastUvIndex {
    pub list: Vec<UvIndex>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct HistoricalUvIndexElement {
    pub lat: f32,
    pub lon: f32,
    pub date_isp: String,
    pub date: u64,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct HistoricalUvIndex {
    pub list: Vec<HistoricalUvIndexElement>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    /// This test/derive once failed
    fn current_weather_derive_failure_1() {
        let der_string = "{\"coord\":{\"lon\":10.45,\"lat\":49.57},\"weather\":[{\"id\":801,\"main\":\"Clouds\",\"description\":\"Ein paar Wolken\",\"icon\":\"02d\"}],\"base\":\"stations\",\"main\":{\"temp\":2.94,\"pressure\":998,\"humidity\":86,\"temp_min\":1.67,\"temp_max\":3.89},\"visibility\":10000,\"wind\":{\"speed\":1},\"clouds\":{\"all\":20},\"dt\":1573810268,\"sys\":{\"type\":1,\"id\":1274,\"country\":\"DE\",\"sunrise\":1573799471,\"sunset\":1573832742},\"timezone\":3600,\"id\":2820859,\"name\":\"Berlin\",\"cod\":200}";

        let weather_report: WeatherReportCurrent =
            serde_json::from_str(&der_string).expect("current weather derive failure testcase 1");

        let weather_report_derived = WeatherReportCurrent {
            coord: Coordinates {
                lon: 10.45,
                lat: 49.57,
            },
            weather: vec![Weather {
                id: 801,
                main: "Clouds".to_string(),
                description: "Ein paar Wolken".to_string(),
                icon: "02d".to_string(),
            }],
            base: "stations".to_string(),
            main: Main {
                temp: 2.94,
                pressure: 998.0,
                humidity: 86.0,
                temp_min: 1.67,
                temp_max: 3.89,
                ..Default::default()
            },
            visibility: Some(10000),
            wind: Wind {
                speed: 1.0,
                ..Default::default()
            },
            clouds: Clouds {
                all: 20,
                ..Default::default()
            },
            dt: 1573810268,
            sys: Sys {
                message_type: 1,
                id: 1274,
                country: "DE".to_string(),
                sunrise: 1573799471,
                sunset: 1573832742,
                ..Default::default()
            },
            timezone: Some(3600),
            id: 2820859,
            name: "Berlin".to_string(),
            cod: 200,

            ..Default::default()
        };

        assert_eq!(weather_report.timezone, Some(3600));
        assert_eq!(
            weather_report.clouds,
            Clouds {
                all: 20,
                ..Default::default()
            }
        );

        assert_eq!(
            weather_report.wind,
            Wind {
                speed: 1.0,
                ..Default::default()
            }
        );

        assert_eq!(weather_report, weather_report_derived);
    }
}
