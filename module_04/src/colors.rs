//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use module_04::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

use rand::random;

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use module_04::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use module_04::colors::*;
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```
/// use module_04::colors::*;
/// println!("{}", blue("Blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code for bold text.
/// # Examples:
/// ```
/// use module_04::colors::*;
/// println!("{}", bold("Bold"));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code to reset the text settings.
/// # Examples:
/// ```
/// use module_04::colors::*;
/// println!("{}", reset("Reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Enum to define the available test attributes 
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// struct to define the respective text attributes for a string
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

impl Default for ColorString {
    fn default() -> Self {
        Self{
            color: Color::Red,
            string: "".to_string(),
            colorized: "".to_string()
        }
    }
}

impl ColorString {
    /// method that will use the string and color fields to create a colorized string and assign it to the colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }
    /// method to write out the current string
    pub fn println(&self) {
        println!("{}", self.colorized);
    }
    /// method that write a string in "normal" text
    pub fn reset(&mut self) {
        self.string="".to_string();
        self.colorized = reset(&self.string);
    }

    /// method that write a string in "random" attributes
    pub fn random(&mut self) {
        let x = random::<u8>() % 4;
        match x {
            0 => self.colorized = red(&self.string),
            1 => self.colorized = green(&self.string),
            2 => self.colorized = blue(&self.string),
            3 => self.colorized = bold(&self.string),
            _ => self.colorized = reset(&self.string),
        }
    }

}
