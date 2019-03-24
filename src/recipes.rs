use crate::items::*;

#[derive(Debug)]
pub struct Ingredient {
    pub item: &'static Item,
    pub quantity: f64,
}

macro_rules! ingredients {
    [$(($item: expr, $qty: expr),)*] => {
        &[$(
            Ingredient {
                item: &$item,
                quantity: $qty,
            },
        )*]
    }
}

#[derive(Debug)]
pub struct Product {
    pub item: &'static Item,
    pub quantity: f64,
}

macro_rules! products {
    [$(($item: expr, $qty: expr),)*] => {
        &[$(
            Product {
                item: &$item,
                quantity: $qty,
            },
        )*]
    }
}

#[derive(Debug)]
pub struct Recipe {
    pub name: &'static str,
    pub ingredients: &'static [Ingredient],
    pub products: &'static [Product],
    pub time: f64,
}

pub static SMELT_IRON_INGOT: Recipe = Recipe {
    name: "Smelt Iron Ingot",
    ingredients: ingredients![
        (IRON_ORE, 30.0),
    ],
    products: products![
        (IRON_INGOT, 30.0),
    ],
    time: 60.0
};

pub static CRAFT_IRON_ROD: Recipe = Recipe {
    name: "Craft Iron Rod",
    ingredients: ingredients![
        (IRON_INGOT, 15.0),
    ],
    products: products![
        (IRON_ROD, 15.0),
    ],
    time: 60.0
};

pub static CRAFT_SCREW: Recipe = Recipe {
    name: "Craft Screw",
    ingredients: ingredients![
        (IRON_ROD, 15.0),
    ],
    products: products![
        (SCREW, 90.0),
    ],
    time: 60.0
};

pub static ASSEMBLE_REINFORCED_IRON_PLATE: Recipe = Recipe {
    name: "Assemble Reinforced Iron Plate",
    ingredients: ingredients![
        (IRON_PLATE, 20.0),
        (SCREW, 120.0),
    ],
    products: products![
        (REINFORCED_IRON_PLATE, 5.0),
    ],
    time: 60.0,
};
