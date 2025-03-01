use module_03::colors::{Color, ColorString};
use module_03::csvwrite::{csvwrite};

fn color_demo() -> i32{
    let mut result:i32 = -1;
    result=0;
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
    result
}

fn csv_demo() -> i32 {
    let result:i32 = 0;
    csvwrite();
    result
}

fn main() {
    let opts = Opts::parse();
    let mut result:i32 = -1;
    if opts.mode.expect("REASON").as_str() == "color" {
        result = color_demo();
    } else if opts.mode.expect("REASON").as_str() == "csv" {
        result = csv_demo();
    }
    result
}
