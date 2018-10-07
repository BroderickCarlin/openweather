#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CityShort {
    pub id: u32,
    pub name: String,
    pub coord: Coordinates,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CityLong {
    pub geoname_id: u64,
    pub name: String,
    pub lat: u32,
    pub lon: u32,
    pub country: String,
    pub iso2: String,
#[serde(rename="type")]
    pub city_type: String,
    pub population: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
    pub humidity: f32,
    pub temp_kf: Option<f32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rain {
#[serde(rename="3h")]
    pub three_h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    pub pod: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempDaily {
    pub day: f32,
    pub min: f32,
    pub max: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSliceHourly {
    pub dt: u64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub rain: Rain,
    pub sys: System,
    pub dt_txt: String
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorReport {
    pub cod: u32, 
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
#[serde(rename="type")]
    pub message_type: u32,
    pub id: u32,
    pub message: f32,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherReportCurrent {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub id: u64,
    pub name: String,
    pub cod: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherReport5Day {
    pub cod: String,
    pub message: f32,
    pub list: Vec<TimeSliceHourly>,
    pub city: CityShort,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherReport16Day {
    pub cod: String,
    pub message: f32,
    pub city: CityLong,
    pub cnt: u8,
    pub list: Vec<TimeSliceDaily>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherReportHistoricalElement {
    pub main: Main,
    pub wind: Wind,
    pub clouds: Clouds,
    pub weather: Vec<Weather>,
    pub dt: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherReportHistorical {
    pub message: String,
    pub cod: String,
    pub city_id: u64,
    pub calctime: f32,
    pub cnt: u32,
    pub list: Vec<WeatherReportHistoricalElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAccumulatedTemperatureElement {
    pub date: String,
    pub temp: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAccumulatedPrecipitation {
    pub message: String,
    pub cod: String,
    pub city_id: u64,
    pub calctime: u64,
    pub list: Vec<WeatherAccumulatedPrecipitationElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAccumulatedPrecipitationElement {
    pub date: String,
    pub rain: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAccumulatedTemperature {
    pub message: String,
    pub cod: String,
    pub city_id: u64,
    pub calctime: u64,
    pub list: Vec<WeatherAccumulatedTemperatureElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UvIndex {
    pub lat: f32,
    pub lon: f32,
    pub data_iso: String,
    pub date: u64,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastUvIndex {
    pub list: Vec<UvIndex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalUvIndexElement {
    pub lat: f32,
    pub lon: f32,
    pub date_isp: String,
    pub date: u64,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalUvIndex {
    pub list: Vec<HistoricalUvIndexElement>,
}
