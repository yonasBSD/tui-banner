use tui_banner::{Align, Banner, ColorMode, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("WARM LUXURY")? // text
        .color_mode(ColorMode::TrueColor) // truecolor
        .gradient(Gradient::vertical(Palette::from_hex(&[
            "#FF5AD9", // pink
            "#FF8FAB", // coral
            "#FFD166", // gold
        ]))) // gradient stops
        .fill(Fill::Keep) // keep glyphs
        .dither()
        .targets("░▒▓") // dither targets
        .checker(3) // checker period
        .align(Align::Center) // center align
        .padding(1) // uniform padding
        .render();

    println!("{banner}");
    Ok(())
}
