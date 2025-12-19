use tui_banner::{Align, Banner, ColorMode, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")? // text
        .color_mode(ColorMode::TrueColor) // truecolor
        .gradient(Gradient::vertical(Palette::from_hex(&[
            "#FACC15", // yellow
            "#FB923C", // orange
            "#EF4444", // red
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
