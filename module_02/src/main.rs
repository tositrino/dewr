use clap::Parser;

use module_02::colors::{Color, ColorString};
use module_02::fruitsalad::{default_fruitlist,read_fruitlist,create_fruitsalad,display_fruitsalad,demo_fruitsalad};
use module_02::datarace::{datarace01,datarace02};


#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "ths <development@ths.one>",
    about = "module 02 lab demos"
)]
struct Opts {
    #[clap(short, long)]
    mode: Option<String>,
    fruits: Option<String>,
    csvfile: Option<String>,
}

fn color_demo( opts : Opts::as_ref ) -> i32{
    let mut result:i32 = -1;
    if opts.mode.expect("REASON").as_str() != "color" {
        return result;
    }   
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

fn fruitsalad_demo( opts : Opts::as_ref )->i32 {
    let mut result:i32 = -1;
    if opts.mode.expect("REASON").as_str() != "fruitsalad" {
        return result;
    }  
    result =0; 
    // fruitlist initialization
    let mut fruit_list = default_fruitlist();
    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Could not read file");
            read_fruitlist(&fruits)
        },
        None => {
            if opts.fruits.is_some() {
              opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
            }
            else {
                default_fruitlist()
            }
        }
    };

    // display fruit salad
    let fruit_salad = create_fruitsalad(fruit_list);
    display_fruitsalad(fruit_salad);

    result
}

fn datarace_demo( opts : Opts::as_ref )->i32 {
    let mut result:i32 = -1;
    if opts.mode.expect("REASON").as_str() != "race" {
        return result;
    }   
    result  = datarace01();
    result += datarace02();
    result
}


fn main() {
    let opts: Opts = Opts::parse();
    // start with color demo if requested
    color_demo(&opts);
    // run fruitsalad demo if requested    
    fruitsalad_demo(&opts);
    // run datarace demo if requested
    datarace_demo(&opts);
}
