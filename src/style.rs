use crate::color::Preset;

/// Named banner styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    /// Neon Cyber (cyan -> purple -> pink).
    NeonCyber,
    /// Arctic Tech (cyan -> blue -> white).
    ArcticTech,
    /// Sunset Neon (orange -> pink -> purple).
    SunsetNeon,
    /// Forest Sky (green -> teal -> blue).
    ForestSky,
    /// Chrome (silver metallic).
    Chrome,
    /// CRT Amber (retro amber).
    CrtAmber,
    /// Ocean Flow (blue -> teal -> aqua).
    OceanFlow,
    /// Deep Space (blue -> purple -> indigo).
    DeepSpace,
    /// Fire Warning (yellow -> orange -> red).
    FireWarning,
    /// Warm Luxury (pink -> coral -> gold).
    WarmLuxury,
    /// Earth Tone (sand -> earth -> olive).
    EarthTone,
    /// Royal Purple (lavender -> purple -> deep purple).
    RoyalPurple,
    /// Matrix (neon green -> deep green).
    Matrix,
    /// Aurora Flux (teal -> sky blue -> violet -> aurora purple).
    AuroraFlux,
}

impl Style {
    pub(crate) fn preset(self) -> Preset {
        match self {
            Style::NeonCyber => Preset::NeonCyber,
            Style::ArcticTech => Preset::ArcticTech,
            Style::SunsetNeon => Preset::SunsetNeon,
            Style::ForestSky => Preset::ForestSky,
            Style::Chrome => Preset::Chrome,
            Style::CrtAmber => Preset::CrtAmber,
            Style::OceanFlow => Preset::OceanFlow,
            Style::DeepSpace => Preset::DeepSpace,
            Style::FireWarning => Preset::FireWarning,
            Style::WarmLuxury => Preset::WarmLuxury,
            Style::EarthTone => Preset::EarthTone,
            Style::RoyalPurple => Preset::RoyalPurple,
            Style::Matrix => Preset::Matrix,
            Style::AuroraFlux => Preset::AuroraFlux,
        }
    }
}
