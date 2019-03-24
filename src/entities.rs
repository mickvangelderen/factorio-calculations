#[derive(Debug)]
pub struct Entity {
    pub name: &'static str,
    pub crafting_speed: f64,
    pub energy_consumption: Option<f64>,
}

pub static MINING_DRILL_MK1: Entity = Entity {
    name: "Mining Drill MK1",
    crafting_speed: 1.0,
    energy_consumption: Some(4.0),
};

pub static MINING_DRILL_MK2: Entity = Entity {
    name: "Mining Drill MK2",
    crafting_speed: 2.0,
    energy_consumption: Some(4.0),
};

pub static SMELTER: Entity = Entity {
    name: "Smelter",
    crafting_speed: 1.0,
    energy_consumption: Some(4.0),
};

pub static CONSTRUCTOR: Entity = Entity {
    name: "Constructor",
    crafting_speed: 1.0,
    energy_consumption: Some(4.0),
};

pub static ASSEMBLER: Entity = Entity {
    name: "Assembler",
    crafting_speed: 1.0,
    energy_consumption: Some(4.0),
};
