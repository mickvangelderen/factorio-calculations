use super::*;

pub static boiler_mk1_burning_charcoal: Processor = Processor {
    name: "boiler_mk1_burning_charcoal",
    speed: 1.0,
    energy_consumption: 3_600_000.0,
    energy_source: Material::Charcoal,
    drain: 0.0,
};

pub static boiler_mk1_burning_carbon: Processor = Processor {
    name: "boiler_mk1_burning_carbon",
    speed: 1.0,
    energy_consumption: 3_600_000.0,
    energy_source: Material::Carbon,
    drain: 0.0,
};

pub static boiler_mk1_burning_wood_pellet: Processor = Processor {
    name: "boiler_mk1_burning_wood_pellet",
    speed: 1.0,
    energy_consumption: 3_600_000.0,
    energy_source: Material::WoodPellet,
    drain: 0.0,
};

pub static boiler_mk2_burning_carbon: Processor = Processor {
    name: "boiler_mk2_burning_carbon",
    speed: 1.0,
    energy_consumption: 3_600_000.0,
    energy_source: Material::Carbon,
    drain: 0.0,
};

pub static hydro_plant_mk1: Processor = Processor {
    name: "hydro_plant_mk1",
    speed: 1.0,
    energy_consumption: 200_000.0,
    energy_source: Material::Joule,
    drain: 6_600.0,
};

pub static clarifier: Processor = Processor {
    name: "clarifier",
    speed: 2.0,
    energy_consumption: 30_000.0,
    energy_source: Material::Joule,
    drain: 1_000.0,
};

pub static crystallizer_mk1: Processor = Processor {
    name: "crystallizer_mk1",
    speed: 1.5,
    energy_consumption: 150_000.0,
    energy_source: Material::Joule,
    drain: 5_000.0,
};

pub static ore_sorting_facility_mk1: Processor = Processor {
    name: "ore_sorting_facility_mk1",
    speed: 0.75,
    energy_consumption: 150_000.0,
    energy_source: Material::Joule,
    // TODO: lookup real drain
    drain: 5_000.0,
};

pub static assembly_machine_mk1: Processor = Processor {
    name: "assembly_machine_mk1",
    speed: 0.5,
    energy_consumption: 100_000.0,
    energy_source: Material::Joule,
    drain: 3_300.0,
};

pub static flare_stack: Processor = Processor {
    name: "flare_stack",
    speed: 2.0,
    energy_consumption: 30_000.0,
    energy_source: Material::Joule,
    drain: 1_000.0,
};

pub static algae_farm_mk1: Processor = Processor {
    name: "algae_farm_mk1",
    speed: 1.0,
    energy_consumption: 120_000.0,
    energy_source: Material::Joule,
    drain: 4_000.0,
};

pub static electrolyser_mk1: Processor = Processor {
    name: "electrolyser_mk1",
    speed: 1.0,
    energy_consumption: 300_000.0,
    energy_source: Material::Joule,
    drain: 10_000.0,
};

pub static liquifier_mk1: Processor = Processor {
    name: "liquifier_mk1",
    speed: 1.5,
    energy_consumption: 125_000.0,
    energy_source: Material::Joule,
    drain: 4_100.0,
};

pub static ore_crusher_mk1: Processor = Processor {
    name: "ore_crusher_mk1",
    speed: 1.5,
    energy_consumption: 100_000.0,
    energy_source: Material::Joule,
    drain: 3_300.0,
};

pub static stone_furnace_burning_charcoal: Processor = Processor {
    name: "stone_furnace_burning_charcoal",
    speed: 1.0,
    energy_consumption: 180_000.0,
    energy_source: Material::Charcoal,
    drain: 0.0,
};

pub static stone_furnace_burning_carbon: Processor = Processor {
    name: "stone_furnace_burning_carbon",
    speed: 1.0,
    energy_consumption: 180_000.0,
    energy_source: Material::Carbon,
    drain: 0.0,
};

pub static offshore_pump: Processor = Processor {
    name: "offshore_pump",
    speed: 1200.0,
    energy_consumption: 0.0,
    energy_source: Material::Joule,
    drain: 0.0,
};

pub static seafloor_pump: Processor = Processor {
    name: "seafloor_pump",
    speed: 300.0,
    energy_consumption: 0.0,
    energy_source: Material::Joule,
    drain: 0.0,
};

pub static washing_plant_mk1: Processor = Processor {
    name: "washing_plant_mk1",
    speed: 1.5,
    energy_consumption: 100_000.0,
    energy_source: Material::Joule,
    drain: 3_300.0,
};
