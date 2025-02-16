use module_01::colors::{Color, ColorString};
use module_01::collections::collections;
use module_01::freq_count::freq_count;
use module_01::lang_weights::lang_weights;
use module_01::cwc::ufc_graph;

fn main() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    color_string.println();

    let mut color_string = ColorString {
        color: Color::Green,
        string: "Green".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    color_string.println();

    let mut color_string = ColorString {
        color: Color::Blue,
        string: "Blue".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    color_string.println();

    let mut color_string = ColorString {
        color: Color::Bold,
        string: "Bold".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    color_string.println();

    let mut color_string = ColorString::default();
    color_string.reset();
    collections();
    freq_count();
    lang_weights();
    ufc_graph();

}
