use iced::Theme;

#[derive(Debug, Default, Clone)]
pub struct ThemeProvider {
    theme_type: ThemeType,
}

impl ThemeProvider {
    pub fn new() -> Self {
        Self {
            theme_type: ThemeType::default(),
        }
    }

    pub fn change_theme(&mut self, theme_type: ThemeType) {
        // todo: add proper theme management + maybe storage
        self.theme_type = theme_type.into();
    }

    pub fn theme(&self) -> Theme {
        self.theme_type.into()
    }
}

impl From<Theme> for ThemeType {
    fn from(value: Theme) -> Self {
        use Theme::*;
        match value {
            Light => Self::Light,
            Dark => Self::Dark,
            Dracula => Self::Dracula,
            Nord => Self::Nord,
            SolarizedLight => Self::SolarizedLight,
            SolarizedDark => Self::SolarizedDark,
            GruvboxLight => Self::GruvboxLight,
            GruvboxDark => Self::GruvboxDark,
            CatppuccinLatte => Self::CatppuccinLatte,
            CatppuccinFrappe => Self::CatppuccinFrappe,
            CatppuccinMacchiato => Self::CatppuccinMacchiato,
            CatppuccinMocha => Self::CatppuccinMocha,
            TokyoNight => Self::TokyoNight,
            TokyoNightStorm => Self::TokyoNightStorm,
            TokyoNightLight => Self::TokyoNightLight,
            KanagawaWave => Self::KanagawaWave,
            KanagawaDragon => Self::KanagawaDragon,
            KanagawaLotus => Self::KanagawaLotus,
            Moonfly => Self::Moonfly,
            Nightfly => Self::Nightfly,
            Oxocarbon => Self::Oxocarbon,
            Custom(_) => todo!(),
        }
    }
}

impl From<ThemeType> for Theme {
    fn from(value: ThemeType) -> Self {
        use ThemeType::*;
        match value {
            Light => Self::Light,
            Dark => Self::Dark,
            Dracula => Self::Dracula,
            Nord => Self::Nord,
            SolarizedLight => Self::SolarizedLight,
            SolarizedDark => Self::SolarizedDark,
            GruvboxLight => Self::GruvboxLight,
            GruvboxDark => Self::GruvboxDark,
            CatppuccinLatte => Self::CatppuccinLatte,
            CatppuccinFrappe => Self::CatppuccinFrappe,
            CatppuccinMacchiato => Self::CatppuccinMacchiato,
            CatppuccinMocha => Self::CatppuccinMocha,
            TokyoNight => Self::TokyoNight,
            TokyoNightStorm => Self::TokyoNightStorm,
            TokyoNightLight => Self::TokyoNightLight,
            KanagawaWave => Self::KanagawaWave,
            KanagawaDragon => Self::KanagawaDragon,
            KanagawaLotus => Self::KanagawaLotus,
            Moonfly => Self::Moonfly,
            Nightfly => Self::Nightfly,
            Oxocarbon => Self::Oxocarbon,
            Custom => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ThemeType {
    Light,
    Dark,
    /// The built-in Dracula variant.
    Dracula,
    /// The built-in Nord variant.
    Nord,
    /// The built-in Solarized Light variant.
    SolarizedLight,
    /// The built-in Solarized Dark variant.
    SolarizedDark,
    /// The built-in Gruvbox Light variant.
    GruvboxLight,
    /// The built-in Gruvbox Dark variant.
    GruvboxDark,
    /// The built-in Catppuccin Latte variant.
    CatppuccinLatte,
    /// The built-in Catppuccin Frappé variant.
    CatppuccinFrappe,
    /// The built-in Catppuccin Macchiato variant.
    CatppuccinMacchiato,
    /// The built-in Catppuccin Mocha variant.
    CatppuccinMocha,
    /// The built-in Tokyo Night variant.
    TokyoNight,
    /// The built-in Tokyo Night Storm variant.
    TokyoNightStorm,
    /// The built-in Tokyo Night Light variant.
    TokyoNightLight,
    /// The built-in Kanagawa Wave variant.
    KanagawaWave,
    /// The built-in Kanagawa Dragon variant.
    KanagawaDragon,
    /// The built-in Kanagawa Lotus variant.
    KanagawaLotus,
    /// The built-in Moonfly variant.
    Moonfly,
    /// The built-in Nightfly variant.
    Nightfly,
    /// The built-in Oxocarbon variant.
    Oxocarbon,
    /// A [`Theme`] that uses a [`Custom`] palette.
    Custom,
}

impl ThemeType {
    /// A list with all the defined themes.
    pub const ALL: &'static [Self] = &[
        Self::Light,
        Self::Dark,
        Self::Dracula,
        Self::Nord,
        Self::SolarizedLight,
        Self::SolarizedDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::CatppuccinLatte,
        Self::CatppuccinFrappe,
        Self::CatppuccinMacchiato,
        Self::CatppuccinMocha,
        Self::TokyoNight,
        Self::TokyoNightStorm,
        Self::TokyoNightLight,
        Self::KanagawaWave,
        Self::KanagawaDragon,
        Self::KanagawaLotus,
        Self::Moonfly,
        Self::Nightfly,
        Self::Oxocarbon,
    ];
}

impl Default for ThemeType {
    fn default() -> Self {
        Self::TokyoNightStorm
    }
}
