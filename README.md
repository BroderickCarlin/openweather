# openweather &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/openweather.svg
[crates.io]: https://crates.io/crates/openweather

**Openweather is an *unofficial* thin wrapper around OpenWeatherMaps API for requesting current and historical weather data**

---

Useful links:
- [OpenWeatherMap API's](https://openweathermap.org/api)
- [Registering an API key](https://openweathermap.org/appid)

## How to use

To request weather data a `LocationSpecifier` needs to be defined for the location of interest. The available methods of specifiying a location are:
- CityAndCountryName (`{city: "CITY_NAME", country: "COUNTRY_CODE"}`)
- CityId (`CITY_ID_CODE`)
- Coordinates (`{lat: LATITUDE, lon: LONGITUDE}`)
- ZipCode (`{zip: "ZIP_CODE", country: "COUNTRY_CODE"}`)
- BoundingBox (`{lon_left: LEFT_LONGITUDE, lat_bottom: BOTTOM_LATITUDE, lon_right: RIGHT_LONGITUDE, lat_top: TOP_LATITUDE}`)
- Circle (`{lat: CENTER_LATITUDE, lon: CENTER_LONGITDE, count: NUMBER_OF_CITIES_OF_INTEREST}`)
- CityIds (`[CITY_ID_1, CITY_ID_2]`)

Once a `LocationSpecifier` has been created it can be used to querry any of available API endpoints: 
- get_current_weather
- get_5_day_forecast
- get_16_day_forecast
- get_historical_data
- get_accumulated_temperature_data
- get_accumulated_precipitation_data
- get_current_uv_index
- get_forecast_uv_index
- get_historical_uv_index

An example of querrying the current temperature in Minneapolis, MN:
```rust
extern crate openweather;

use openweather::LocationSpecifier;
static API_KEY: &str = "YOUR_API_KEY_HERE";

fn main() 
{
    let loc = LocationSpecifier::CityAndCountryName{city:"Minneapolis", country:"USA"};
    let weather = openweather::get_current_weather(loc, API_KEY).unwrap();
    println!("Right now in Minneapolis, MN it is {}K", weather.main.temp);
}
```

## License

openweather is licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)