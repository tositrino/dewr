//! This module contains functions for the fruitsalad  lab
//! # Examples:
//! ````
//! use module_02::fruitsalad::*;
//! demo_fruitsalad();
//! ````

use rand::seq::SliceRandom;
use rand::rng;

/// create default fruitlist 
/// # Examples:
/// ````
/// use module_02::fruitsalad::default_fruitlist;
/// let fruitlist = default_fruitlist(100);
/// ````
pub fn default_fruitlist() -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];
    fruits
}
/// read fruit list from csv
/// # Examples:
/// ````
/// use module_02::fruitsalad::{read_fruitlist};
/// let mut fruitlist = read_fruitlist("fruitlist.csv");
/// ````
pub fn read_fruitlist(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

/// create fruitsalad vector from given input list
/// # Examples:
/// ````
/// use module_02::fruitsalad::{default_fruitlist,create_fruitsalad};
/// let mut fruitlist = default_fruitlist();
/// let fruitsalad = create_fruitsalad(fruitlist);
/// ````
pub fn create_fruitsalad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = rng();
    fruits.shuffle(&mut rng);
    fruits
}

/// fruitsalad display function
/// # Examples:
/// ````
/// use module_02::fruitsalad::{default_fruitlist,display_fruitsalad};
/// fruits = default_fruitlist();
/// display_fruitsalad(fruits);
/// ````
pub fn display_fruitsalad(fruits: Vec<String>)->i32 {
    let result:i32=0;
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
    result
}

/// fruitsalad demo function
/// # Examples:
/// ````
/// use module_02::fruitsalad::demo_fruitsalad;
/// demo_fruitsalad();
/// ````

pub fn demo_fruitsalad()->i32 {
    let result:i32 = 0;
    let fruit_list = default_fruitlist();
    let mut fruit_salad = create_fruitsalad(fruit_list);

    println!("Original fruit salad: {:?}", fruit_salad);
    fruit_salad.push("figs".to_string());
    println!("Modified fruit salad: {:?}", fruit_salad);
    let last:String = fruit_salad.pop().expect("There is no fruit to pop");
    println!("Modified fruit salad: {:?}", fruit_salad);
    println!(" popped fruit = {:?}", last);
    result
}
