use crate::{
    dish::Dish,
    ingredient::{self, Ingredient, MesurementUnit},
};

pub struct GroceryList {
    dishes: Vec<Dish>,
}

impl GroceryList {
    pub fn add_dish(&mut self, dish: Dish) {
        self.dishes.push(dish)
    }
    fn create_list_of_all_ingredients(&self) -> Vec<Ingredient> {
        let mut temp_vec: Vec<Ingredient> = Vec::new();

        // push all ingredients of all dishes into the temp_vec
        for dish in &self.dishes {
            for ingredient in dish.ingredients.clone() {
                temp_vec.push(ingredient)
            }
        }

        let merged_list = Self::merge_ingredient_duplicates(&temp_vec);

        return merged_list;
    }

    /// this will return a vec containing  trupels that represents
    /// the name of the grocery, the mesurement unit (dl, l, g or kg) and the amount needed.
    pub fn get(&self) -> Vec<Ingredient> {
        return self.create_list_of_all_ingredients();
    }

    fn merge_all_of_same_ingredient(
        name: String,
        mesurement_unit: MesurementUnit,
        ingredients: Vec<Ingredient>,
    ) -> Ingredient {
        let mut amount: f32 = 0.0;
        let mesurement_unit: MesurementUnit = mesurement_unit;

        for ingredient in ingredients {
            if ingredient.name == name {
                amount += ingredient.amount;
            }
        }

        return Ingredient {
            name,
            mesurement_unit,
            amount,
        };
    }

    fn merge_ingredient_duplicates(ingredients: &Vec<Ingredient>) -> Vec<Ingredient> {
        let mut merged_ingredients: Vec<Ingredient> = Vec::new();

        for ingredient in ingredients {
            let mut already_merged = false;
            for merged_ingredient in &mut merged_ingredients {
                if merged_ingredient.name == ingredient.name {
                    already_merged = true;
                }
            }

            if !already_merged {
                merged_ingredients.push(Self::merge_all_of_same_ingredient(
                    ingredient.name.clone(),
                    ingredient.mesurement_unit.clone(),
                    ingredients.clone(),
                ));
            }
        }

        return merged_ingredients;
    }

    pub fn new() -> Self {
        GroceryList {
            dishes: Vec::new(),
        }
    }
}
