use crate::items;

#[derive(Debug)]
pub struct Ingredient {
    pub item: &'static items::Item,
    pub quantity: f64,
}

#[derive(Debug)]
pub struct Product {
    pub item: &'static items::Item,
    pub quantity: f64,
}

#[derive(Debug)]
pub struct Recipe {
    pub name: &'static str,
    pub ingredients: &'static [Ingredient],
    pub products: &'static [Product],
    pub time: f64,
}

pub static IRON_INGOT: Recipe = Recipe {
    name: "Iron Ingot",
    ingredients: &[
        Ingredient {
            item: &items::IRON_ORE,
            quantity: 30.0,
        },
    ],
    products: &[
        Product {
            item: &items::IRON_INGOT,
            quantity: 30.0,
        },
    ],
    time: 60.0
};

pub static IRON_ROD: Recipe = Recipe {
    name: "Iron Rod",
    ingredients: &[
        Ingredient {
            item: &items::IRON_INGOT,
            quantity: 15.0,
        },
    ],
    products: &[
        Product {
            item: &items::IRON_ROD,
            quantity: 15.0,
        },
    ],
    time: 60.0
};
