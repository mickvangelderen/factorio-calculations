pub struct Item {
    name: &'static str,
}

pub struct Fluid {
    name: &'static str,
}

pub enum Ingredient {
    Item {
        name: &'static str,
        amount: u32,
    },
    Fluid {
        name: &'static str,
        amount: u32,
    },
    FluidTemp {
        name: &'static str,
        amount: u32,
        minimum_temperature: u32,
        maximum_temperature: u32,
    },
}

pub enum Product {
    Item {
        name: &'static str,
        amount: f32,
    },
    ItemChance {
        name: &'static str,
        amount_min: u32,
        amount_max: u32,
        probability: f64,
    },
    Fluid {
        name: &'static str,
        amount: f32,
    },
    FluidChance {
        name: &'static str,
        amount_min: u32,
        amount_max: u32,
        probability: f64,
    },
    FluidTemp {
        name: &'static str,
        amount: f32,
        temperature: u32,
    },
    FluidChanceTemp {
        name: &'static str,
        amount_min: u32,
        amount_max: u32,
        probability: f64,
        temperature: u32,
    },
}
