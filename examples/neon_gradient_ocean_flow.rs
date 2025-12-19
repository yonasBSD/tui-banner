use tui_banner::{Banner, Style};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")? // text
        .style(Style::OceanFlow) // style
        .render();

    println!("{banner}");
    Ok(())
}
