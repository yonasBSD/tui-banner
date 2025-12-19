/// Supported color types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    /// 24-bit RGB color.
    Rgb(u8, u8, u8),
    /// ANSI 256-color palette index.
    Ansi256(u8),
}

/// Color output mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    /// Auto-detect from environment.
    Auto,
    /// 24-bit truecolor output.
    TrueColor,
    /// 256-color output.
    Ansi256,
    /// Disable color output.
    NoColor,
}

/// Palette of colors for gradients.
#[derive(Clone, Debug)]
pub struct Palette {
    colors: Vec<Color>,
}

/// Named palette presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
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
}

impl Palette {
    /// Create a palette from colors.
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    /// Create a palette from hex strings (invalid entries are ignored).
    pub fn from_hex(hexes: &[&str]) -> Self {
        let mut colors = Vec::with_capacity(hexes.len());
        for hex in hexes {
            if let Some(color) = parse_hex_color(hex) {
                colors.push(color);
            }
        }
        Self { colors }
    }

    /// Create a palette from a named preset.
    pub fn preset(preset: Preset) -> Self {
        Self::from_hex(preset.hexes())
    }

    /// Get palette colors.
    pub fn colors(&self) -> &[Color] {
        &self.colors
    }
}

impl Color {
    /// Linear interpolation between colors.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        match (self, other) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let t = t.clamp(0.0, 1.0);
                let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t).round() as u8;
                let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t).round() as u8;
                let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t).round() as u8;
                Color::Rgb(r, g, b)
            }
            (left, _) => left,
        }
    }
}

fn parse_hex_color(input: &str) -> Option<Color> {
    let hex = input.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

impl Preset {
    fn hexes(self) -> &'static [&'static str] {
        match self {
            Preset::NeonCyber => &["#00E5FF", "#7B5CFF", "#FF5AD9"],
            Preset::ArcticTech => &["#00E5FF", "#3A7BFF", "#E6F6FF"],
            Preset::SunsetNeon => &["#FF8A00", "#FF4FD8", "#7B5CFF"],
            Preset::ForestSky => &["#22C55E", "#14B8A6", "#2563EB"],
            Preset::Chrome => &["#F5F5F5", "#BDBDBD", "#6B7280", "#E5E7EB"],
            Preset::CrtAmber => &["#FFB000", "#FF8C00", "#7A3E00"],
            Preset::OceanFlow => &["#2563EB", "#0EA5A4", "#5EEAD4"],
            Preset::DeepSpace => &["#1E3A8A", "#5B21B6", "#312E81"],
            Preset::FireWarning => &["#FACC15", "#FB923C", "#EF4444"],
            Preset::WarmLuxury => &["#FF5AD9", "#FF8FAB", "#FFD166"],
            Preset::EarthTone => &["#E6CCB2", "#B08968", "#6B705C"],
            Preset::RoyalPurple => &["#E9D5FF", "#A855F7", "#581C87"],
            Preset::Matrix => &["#00FF9C", "#00C46A", "#003B24"],
        }
    }
}
