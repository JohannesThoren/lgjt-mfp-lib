use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ingredient {
    pub name: String,
    pub mesurement_unit: MesurementUnit,
    pub amount: f32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MesurementUnit {
    L = 1,  // Liter
    DL = 2, // Deciliter
    KG = 3, // Kilogram
    G = 4,  // Gram
}

impl Ingredient {
    pub fn new(name: String, mesurement_unit: MesurementUnit, amount: f32) -> Self {
        Ingredient {
            name,
            mesurement_unit,
            amount,
        }
    }
}
