/// Enum to represent the different platforms 
/// 
/// Defaults to "Platform::PC"
#[derive(PartialEq, Eq, Debug)]
pub enum Platform {
    PC, PS4, XBox, Switch
}

impl Default for Platform {
    fn default() -> Self {
        Platform::PC
    }
}

impl std::string::ToString for Platform {
    fn to_string(&self) -> String {
        let into: &str = self.into();
        into.to_string()
    }
}

impl Into<&str> for &Platform {
    fn into(self) -> &'static str {
        match self {
            Platform::PC => "pc",
            Platform::PS4 => "ps4",
            Platform::XBox => "xb1",
            Platform::Switch => "swi",
        }
    }
}

impl Into<&str> for Platform {
    fn into(self) -> &'static str {
        (&self).into()        
    }
}

/// Enum representing the different languages
/// 
/// Defaults to `Language::English`
#[derive(PartialEq, Eq, Debug)]
pub enum Language {
    English, German, Spanish, French, Italian, Korean, Polish, Portuguese, Russian, Chinese, Ukrainian
}

impl Into<&str> for &Language {
    fn into(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Italian => "it",
            Language::Korean => "ko",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Chinese => "zh",
            Language::Ukrainian => "uk",
        }
    }
}

impl Into<&str> for Language {
    fn into(self) -> &'static str {
        (&self).into()
    }
}

impl std::string::ToString for Language {
    fn to_string(&self) -> String {
        let into: &str = self.into();
        into.to_string()
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}