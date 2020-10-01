use termion::color;

pub fn main() {
    println!("{red}more red than any comrade{reset}",
             red   = color::Fg(color::Red),
             reset = color::Fg(color::Reset));
}