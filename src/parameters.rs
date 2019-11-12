#[derive(Default, Debug)]
pub struct Settings {
    pub unit: Option<Unit>,
    pub lang: Option<Language>,
}

impl Settings {
    pub fn format(&self) -> Vec<(String, String)> {
        let mut res: Vec<(String, String)> = Vec::new();
        add_param(&mut res, self.unit);
        add_param(&mut res, self.lang);
        res
    }
}

fn add_param<T: FormatParameters>(settings: &mut Vec<(String, String)>, param: Option<T>) {
    param.map(|v: T| v.format().map(|u: (String, String)| settings.push(u)));
}

trait FormatParameters {
    fn format(&self) -> Option<(String, String)>;
}

#[derive(Debug, Clone, Copy)]
pub enum Unit {
    // Celcius, default
    Metric,
    // Kelvin
    Standard,
    // Fahrenheit
    Imperial,
}

impl FormatParameters for Unit {
    fn format(&self) -> Option<(String, String)> {
        match self {
            Unit::Metric => Some(("units".to_string(), "metric".to_string())),
            Unit::Standard => None,
            Unit::Imperial => Some(("units".to_string(), "imperial".to_string())),
        }
    }
}

/// Translation is only applied for the description field!
#[derive(Debug, Clone, Copy)]
pub enum Language {
    Arabic,
    Bulgarian,
    Catalan,
    Czech,
    German,
    Greek,
    English,
    PersianFarsi,
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
    ChineseSimplified,
    ChineseTraditional,
}

impl FormatParameters for Language {
    fn format(&self) -> Option<(String, String)> {
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
                PersianFarsi => "fa",
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
                ChineseSimplified => "zh_cn",
                ChineseTraditional => "zh_tw",
            }
            .to_string(),
        ))
    }
}
