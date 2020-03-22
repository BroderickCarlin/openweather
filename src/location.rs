#[derive(Debug)]
pub enum LocationSpecifier {
    CityAndCountryName {
        city: String,
        country: String,
    },
    CityId(String),
    Coordinates {
        lat: f32,
        lon: f32,
    },
    ZipCode {
        zip: String,
        country: String,
    },

    // The following location specifiers are used to specify multiple cities or a region
    BoundingBox {
        lon_left: f32,
        lat_bottom: f32,
        lon_right: f32,
        lat_top: f32,
        zoom: f32,
    },
    Circle {
        lat: f32,
        lon: f32,
        count: u16,
    },
    CityIds(Vec<String>),
}

impl LocationSpecifier {
    pub fn format(&self) -> Vec<(String, String)> {
        match &self {
            LocationSpecifier::CityAndCountryName { city, country } => {
                if *country == "" {
                    return vec![("q".to_string(), city.to_string())];
                } else {
                    return vec![("q".to_string(), format!("{},{}", city, country))];
                }
            }
            LocationSpecifier::CityId(id) => {
                return vec![("id".to_string(), id.to_string())];
            }
            LocationSpecifier::Coordinates { lat, lon } => {
                return vec![
                    ("lat".to_string(), format!("{}", lat)),
                    ("lon".to_string(), format!("{}", lon)),
                ];
            }
            LocationSpecifier::ZipCode { zip, country } => {
                if *country == "" {
                    return vec![("zip".to_string(), zip.to_string())];
                } else {
                    return vec![("zip".to_string(), format!("{},{}", zip, country))];
                }
            }
            LocationSpecifier::BoundingBox {
                lon_left,
                lat_bottom,
                lon_right,
                lat_top,
                zoom,
            } => {
                return vec![(
                    "bbox".to_string(),
                    format!(
                        "{},{},{},{},{}",
                        lon_left, lat_bottom, lon_right, lat_top, zoom
                    ),
                )];
            }
            LocationSpecifier::Circle { lat, lon, count } => {
                return vec![
                    ("lat".to_string(), format!("{}", lat)),
                    ("lon".to_string(), format!("{}", lon)),
                    ("cnt".to_string(), format!("{}", count)),
                ];
            }
            LocationSpecifier::CityIds(ids) => {
                let mut locations: String = "".to_string();
                for loc in ids {
                    locations += loc;
                }
                return vec![("id".to_string(), locations.to_string())];
            }
        }
    }
}
