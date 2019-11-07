#[derive(Default, Debug)]
pub struct Settings {
    pub unit : Option<Unit>,
    pub lang: Option<Language>,
}

impl Settings{
    pub fn format(&self) -> Vec<(String, String)> {
        let mut res: Vec<(String, String)> = Vec::new();
        self.unit.map(|v: Unit | v.format().map(|u: (String, String) | res.push(u)));
        self.lang.map(|v: Language | v.format().map(|u: (String, String) | res.push(u)));
        res
    }
}

trait Format_Params {
    fn format(&self) -> Option<(String, String)>;
}

fn add_param<T: Format_Params>(settings: &mut Vec<(String, String), param: Option<T>) {
    param.map
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

impl Format_Params for Unit {
    pub fn format(&self) -> Option<(String, String)> {
        match self {
            Unit::Metric => Some(("units".to_string(), "metric".to_string())),
            Unit::Standard => None,
            Unit::Imperial => Some(("units".to_string(), "imperial".to_string())),
        }
    }
}

/// Translation is only applied for the description field!
#[derive(Debug)]
pub enum Language {
    Arabic,
    Bulgarian,
    Catalan,
    Czech,
    German,
    Greek,
    English,
    Persian_Farsi,
    Finnish,
    French,
    Galician,
    Croatian,
    Hungarian,
    Italian,
    Japanese,
    Korean,
    Latvian,
    Lithuanian,
    Macedonian,
    Dutch,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Swedish,
    Slovak,
    Slovenian,
    Spanish,
    Turkish,
    Ukrainian,
    Vietnamese,
    Chinese_Simplified,
    Chinese_Traditional,
}

impl Format_Params for Language {
    pub fn format(&self) -> Option<(String, String)> {
        use Language::*;
        Some((
            String::from("lang"),
            match self {
                Arabic => "ar",
                Bulgarian => "bg",
                Catalan => "ca",
                Czech => "cz",
                German => "de",
                Greek => "el",
                English => "en",
                Persian_Farsi => "fa",
                Finnish => "fi",
                French => "fr",
                Galician => "gl",
                Croatian => "hr",
                Hungarian => "hu",
                Italian => "it",
                Japanese => "ja",
                Korean => "kr",
                Latvian => "la",
                Lithuanian => "lt",
                Macedonian => "mk",
                Dutch => "nl",
                Polish => "pl",
                Portuguese => "pt",
                Romanian => "ro",
                Russian => "ru",
                Swedish => "se",
                Slovak => "sk",
                Slovenian => "sl",
                Spanish => "es",
                Turkish => "tr",
                Ukrainian => "ua",
                Vietnamese => "vi",
                Chinese_Simplified => "zh_cn",
                Chinese_Traditional => "zh_tw",
            }
            .to_string(),
        ))
    }
}