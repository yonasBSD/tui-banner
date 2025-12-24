use tui_banner::{Align, Banner, Frame, FrameStyle, Gradient, Palette, Style};

fn main() -> Result<(), tui_banner::BannerError> {
    let frame_gradient =
        Gradient::horizontal(Palette::from_hex(&["#00E5FF", "#7B5CFF", "#FF5AD9"]));

    let banner = Banner::new("RUST CLI")?
        .style(Style::NeonCyber)
        .align(Align::Center)
        .padding(1)
        .frame(Frame::new(FrameStyle::Rounded).gradient(frame_gradient))
        .render();

    println!("{banner}");
    Ok(())
}
