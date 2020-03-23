use openweather::{LocationSpecifier, Settings};
static API_KEY: &str = "YOUR_API_KEY_HERE";

fn main() {
    let loc = LocationSpecifier::CityAndCountryName {
        city: "Minneapolis".to_string(),
        country: "USA".to_string(),
    };
    match openweather::get_current_weather(&loc, API_KEY, &Settings::default()) {
        Ok(weather) => println!("Right now in Minneapolis, MN it is {}°K", weather.main.temp),
        Err(e) => println!("Error getting weather: {:?}", e),
    }
}
