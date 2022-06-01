use std::fs;

use serde::Deserialize;
use crate::ingredient::Ingredient;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dish {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}


impl Dish {
    pub fn new(name: String, ingredients: Vec<Ingredient>) -> Self {
        Dish { name, ingredients }
    }

    pub fn from_json(path : &str) -> Self {
        let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
        let dish: Dish = serde_json::from_str(contents.as_str()).expect("JSON was not well-formatted");
        println!("{:?}", dish);
        return dish;
    }
}
