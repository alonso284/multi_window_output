#[cfg(test)]
mod color_test {
    use crate::*;
    #[test]
    fn sample_code(){
        let mut screen = Screen::new();

        // Change screen's color
        screen.set_screen_color(Color::Green);

        // Change window's color
        screen.set_window_color(0, Color::Yellow).unwrap();
    }
}

/// `Color`s are used to change `Screen`'s and `Screen` windows' colors.
/// You can change the `Screen`'s and window's colors with `set_screen_color` of `set_window_color`
/// methods.
///
/// ```ignore
/// use multi_window_output::{Screen, Color};
///
/// let mut screen = Screen::new();
///
/// // Change screen's color
/// screen.set_screen_color(Color::Green);
///
/// // Change window's color
/// screen.set_window_color(0, Color::Yellow).unwrap();
/// ```
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    Null, Black, Blue, Cyan, 
    Green, LightBlack, LightBlue, LightCyan, 
    LightGreen, LightMagenta, LightRed, LightWhite, 
    LightYellow, Magenta, Red, White, Yellow,
}

pub fn color_code(color: &Color) -> &'static str {
    match color {
        Color::Black            => termion::color::Black.bg_str(),
        Color::Blue             => termion::color::Blue.bg_str(),
        Color::Cyan             => termion::color::Cyan.bg_str(),
        Color::Green            => termion::color::Green.bg_str(),
        Color::LightBlack       => termion::color::LightBlack.bg_str(),
        Color::LightBlue        => termion::color::LightBlue.bg_str(),
        Color::LightCyan        => termion::color::LightCyan.bg_str(),
        Color::LightGreen       => termion::color::LightGreen.bg_str(),
        Color::LightMagenta     => termion::color::LightMagenta.bg_str(),
        Color::LightRed         => termion::color::LightRed.bg_str(),
        Color::LightWhite       => termion::color::LightWhite.bg_str(),
        Color::LightYellow      => termion::color::LightYellow.bg_str(),
        Color::Magenta          => termion::color::Magenta.bg_str(),
        Color::Red              => termion::color::Red.bg_str(),
        Color::White            => termion::color::White.bg_str(),
        Color::Yellow           => termion::color::Yellow.bg_str(),
        Color::Null             => "",
    }
}
