mod dish;
mod grocery_list;
mod ingredient;
#[cfg(test)]
mod tests {
    use crate::{
        dish::{self, Dish}, grocery_list::{self, GroceryList},
        ingredient::{self, MesurementUnit},
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn new_ingredient() {
        let milk = ingredient::Ingredient::new("milk".to_string(), MesurementUnit::DL, 10.0);
    }

    #[test]
    fn grocery_list() {
        let pasta =
            ingredient::Ingredient::new("Pasta".to_string(), ingredient::MesurementUnit::G, 250.0);
        let tomato_sauce = ingredient::Ingredient::new(
            "TomatoSauce".to_string(),
            ingredient::MesurementUnit::G,
            100.0,
        );

        let pasta_and_tomata_sauce = dish::Dish::new(
            "Pasta And Tomato Sauce".to_string(),
            vec![pasta.clone(), pasta.clone(), tomato_sauce],
        );

        let mut list = grocery_list::GroceryList::new();
        list.add_dish(pasta_and_tomata_sauce);

        let g_list = list.get();

        println!("list: {:?}", g_list)
    }

    #[test]
    fn from_json() {
        let mut grocery_list = GroceryList::new();
        let dish_1 = Dish::from_json("pmm.json");
        let dish_2 = Dish::from_json("pts.json");

        grocery_list.add_dish(dish_1);
        grocery_list.add_dish(dish_2);

        println!("{:?}", grocery_list.get())
    }
}
