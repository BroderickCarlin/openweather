use openweather::{LocationSpecifier, Settings};
static API_KEY: &str = "YOUR_API_KEY_HERE";

fn main() {
    let loc = LocationSpecifier::CityAndCountryName {
        city: "Minneapolis",
        country: "USA",
    };
    let weather = openweather::get_current_weather(&loc, API_KEY, &Settings::default()).unwrap();
    println!("Right now in Minneapolis, MN it is {}K", weather.main.temp);
}
