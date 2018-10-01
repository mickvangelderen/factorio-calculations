pub struct Item {
    pub name: &'static str,
    pub fuel_value: f64,
}

pub struct Fluid {
    pub name: &'static str,
    pub fuel_value: f64,
}

pub enum EntityEnergy {
    None,
    Fueled {
        usage: f64,
        effectivity: f64,
    },
    Electric {
        usage: f64,
        drain: f64,
    },
}

pub struct Entity {
    pub name: &'static str,
    pub crafting_speed: Option<f64>,
    pub energy: EntityEnergy,
}

pub enum Ingredient {
    Item {
        name: &'static str,
        amount: f64,
    },
    Fluid {
        name: &'static str,
        amount: f64,
    },
    FluidTemp {
        name: &'static str,
        amount: f64,
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
        amount_min: f64,
        amount_max: f64,
        probability: f64,
    },
    Fluid {
        name: &'static str,
        amount: f32,
    },
    FluidChance {
        name: &'static str,
        amount_min: f64,
        amount_max: f64,
        probability: f64,
    },
    FluidTemp {
        name: &'static str,
        amount: f64,
        temperature: u32,
    },
    FluidChanceTemp {
        name: &'static str,
        amount_min: f64,
        amount_max: f64,
        probability: f64,
        temperature: u32,
    },
}

pub struct Recipe {
    pub name: &'static str,
    pub ingredients: &'static[Ingredient],
    pub products: &'static[Product],
    pub time: f64,
}
