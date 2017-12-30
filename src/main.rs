#![allow(unused_variables)]

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Material {
    Water,
    MineralizedWater,
    CrushedStone,
    GreenAlgae,
    Fiber,
    WoodPellet,
    WoodBrick,
    Coal,
    CrushedCoal,
    Coke,
    CokePellet,
    Carbon,
    Slag,
    Hydrogen,
    Oxygen,
    CarbonDioxide,
    Joule,
}

impl Material {
    fn joules(&self) -> f64 {
        use Material::*;

        match *self {
            Fiber => 1_000_000.0,
            WoodPellet => 12_000_000.0,
            Coal => 8_000_000.0,
            Coke => 5_000_000.0,
            CokePellet => 30_000_000.0,
            Carbon => 6_000_000.0,
            Joule => 1.0,
            _ => 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Ingredient {
    material: Material,
    quantity: f64,
}

#[derive(Debug)]
pub struct Process {
    ingredients: Vec<Ingredient>,
    time: f64,
}

#[derive(Debug)]
pub struct Processor {
    speed: f64,
    energy_consumption: f64,
    energy_source: Material,
    drain: f64,
}

fn main() {
    // Processors.
    let boiler_mk1 = Processor {
        speed: 3_600_000.0,
        energy_consumption: 1_800_000.0,
        energy_source: Material::Joule,
        drain: 0.0,
    };

    let assembly_machine_mk1 = Processor {
        speed: 0.5,
        energy_consumption: 100_000.0,
        energy_source: Material::Joule,
        drain: 3_300.0,
    };

    let flare_stack = Processor {
        speed: 2.0,
        energy_consumption: 30_000.0,
        energy_source: Material::Joule,
        drain: 1_000.0,
    };

    let algae_farm_mk1 = Processor {
        speed: 1.0,
        energy_consumption: 120_000.0,
        energy_source: Material::Joule,
        drain: 4_000.0,
    };

    let electrolyser_mk1 = Processor {
        speed: 1.0,
        energy_consumption: 300_000.0,
        energy_source: Material::Joule,
        drain: 10_000.0,
    };

    let liquifier_mk1 = Processor {
        speed: 1.5,
        energy_consumption: 125_000.0,
        energy_source: Material::Joule,
        drain: 4_100.0,
    };

    let ore_crusher_mk1 = Processor {
        speed: 1.5,
        energy_consumption: 100_000.0,
        energy_source: Material::Joule,
        drain: 3_300.0,
    };

    let stone_oven_burning_carbon = Processor {
        speed: 1.0,
        energy_consumption: 180_000.0,
        energy_source: Material::Carbon,
        drain: 0.0,
    };

    let offshore_pump = Processor {
        speed: 1200.0,
        energy_consumption: 0.0,
        energy_source: Material::Joule,
        drain: 0.0,
    };

    // Processes.
    let water_pumping = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Water,
                quantity: 1.0,
            },
        ],
        time: 1.0,
    };

    let dirt_water_electrolysis = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Water,
                quantity: -100.0,
            },
            Ingredient {
                material: Material::Oxygen,
                quantity: 30.0,
            },
            Ingredient {
                material: Material::Hydrogen,
                quantity: 40.0,
            },
            Ingredient {
                material: Material::Slag,
                quantity: 1.0,
            },
        ],
        time: 1.0,
    };

    let stone_crushing = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Slag,
                quantity: -1.0,
            },
            Ingredient {
                material: Material::CrushedStone,
                quantity: 2.0,
            },
        ],
        time: 1.0,
    };

    let water_mineralization = Process {
        ingredients: vec![
            Ingredient {
                material: Material::CrushedStone,
                quantity: -10.0,
            },
            Ingredient {
                material: Material::Water,
                quantity: -100.0,
            },
            Ingredient {
                material: Material::MineralizedWater,
                quantity: 100.0,
            },
        ],
        time: 1.0,
    };

    let green_algae_growing = Process {
        ingredients: vec![
            Ingredient {
                material: Material::MineralizedWater,
                quantity: -100.0,
            },
            Ingredient {
                material: Material::CarbonDioxide,
                quantity: -100.0,
            },
            Ingredient {
                material: Material::GreenAlgae,
                quantity: 40.0,
            },
        ],
        time: 20.0,
    };

    let green_algae_to_fiber = Process {
        ingredients: vec![
            Ingredient {
                material: Material::GreenAlgae,
                quantity: -10.0,
            },
            Ingredient {
                material: Material::Fiber,
                quantity: 5.0,
            },
        ],
        time: 3.0,
    };

    let fiber_to_wood_pellet = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Fiber,
                quantity: -12.0,
            },
            Ingredient {
                material: Material::WoodPellet,
                quantity: 2.0,
            },
        ],
        time: 4.0,
    };

    let wood_pellet_to_wood_brick = Process {
        ingredients: vec![
            Ingredient {
                material: Material::WoodPellet,
                quantity: -8.0,
            },
            Ingredient {
                material: Material::WoodBrick,
                quantity: 4.0,
            },
        ],
        time: 2.0,
    };

    let wood_brick_to_coal = Process {
        ingredients: vec![
            Ingredient {
                material: Material::WoodBrick,
                quantity: -1.0,
            },
            Ingredient {
                material: Material::Coal,
                quantity: 3.0,
            },
        ],
        time: 3.5,
    };

    let coal_to_carbon_dioxide = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Coal,
                quantity: -1.0,
            },
            Ingredient {
                material: Material::CarbonDioxide,
                quantity: 50.0,
            },
        ],
        time: 2.0,
    };

    let coal_to_crushed_coal = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Coal,
                quantity: -1.0,
            },
            Ingredient {
                material: Material::CrushedCoal,
                quantity: 2.0,
            },
        ],
        time: 1.0,
    };

    let burn_crushed_coal_to_coke = Process {
        ingredients: vec![
            Ingredient {
                material: Material::CrushedCoal,
                quantity: -1.0,
            },
            Ingredient {
                material: Material::Coke,
                quantity: 2.0,
            },
        ],
        time: 1.0,
    };

    let coke_to_carbon = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Coke,
                quantity: -2.0,
            },
            Ingredient {
                material: Material::CarbonDioxide,
                quantity: -35.0,
            },
            Ingredient {
                material: Material::Carbon,
                quantity: 3.0,
            },
        ],
        time: 2.0,
    };

    let carbon_to_steam = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Carbon,
                quantity: -1.0/Material::Carbon.joules(),
            },
            Ingredient {
                material: Material::Joule,
                quantity: 1.0,
            },
        ],
        time: 1.0,
    };

    let burn_oxygen = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Oxygen,
                quantity: -100.0,
            },
        ],
        time: 1.0,
    };

    let burn_hydrogen = Process {
        ingredients: vec![
            Ingredient {
                material: Material::Hydrogen,
                quantity: -100.0,
            },
        ],
        time: 1.0,
    };

    // Do things.
    let groups = vec![
        Group {
            quantity: 3.0,
            processor: &offshore_pump,
            process: &water_pumping,
        },
        Group {
            quantity: 675.0 / (flare_stack.speed * 100.0),
            processor: &flare_stack,
            process: &burn_oxygen,
        },
        Group {
            quantity: 900.0 / (flare_stack.speed * 100.0),
            processor: &flare_stack,
            process: &burn_hydrogen,
        },
        Group {
            quantity: 22.5,
            processor: &electrolyser_mk1,
            process: &dirt_water_electrolysis,
        },
        Group {
            quantity: 15.0,
            processor: &ore_crusher_mk1,
            process: &stone_crushing,
        },
        Group {
            quantity: 3.0,
            processor: &liquifier_mk1,
            process: &water_mineralization,
        },
        Group {
            quantity: 90.0,
            processor: &algae_farm_mk1,
            process: &green_algae_growing,
        },
        Group {
            quantity: 180.0 / (liquifier_mk1.speed / green_algae_to_fiber.time * 10.0),
            processor: &liquifier_mk1,
            process: &green_algae_to_fiber,
        },
        Group {
            quantity: 90.0 / (assembly_machine_mk1.speed / fiber_to_wood_pellet.time * 12.0),
            processor: &assembly_machine_mk1,
            process: &fiber_to_wood_pellet,
        },
        Group {
            quantity: 15.0 / (assembly_machine_mk1.speed / wood_pellet_to_wood_brick.time * 8.0),
            processor: &assembly_machine_mk1,
            process: &wood_pellet_to_wood_brick,
        },
        Group {
            quantity: 7.5 / (stone_oven_burning_carbon.speed / wood_brick_to_coal.time * 1.0),
            processor: &stone_oven_burning_carbon,
            process: &wood_brick_to_coal,
        },
        Group {
            // Desired 450 for algae farms.
            quantity: (450.0 + 350.0) / (liquifier_mk1.speed / coal_to_carbon_dioxide.time * 50.0),
            processor: &liquifier_mk1,
            process: &coal_to_carbon_dioxide,
        },
        Group {
            // Say we use only 5 coal.
            quantity: 5.0 / 1.0 * coal_to_crushed_coal.time / ore_crusher_mk1.speed,
            processor: &ore_crusher_mk1,
            process: &coal_to_crushed_coal,
        },
        Group {
            quantity: 10.0 / 1.0 * burn_crushed_coal_to_coke.time / stone_oven_burning_carbon.speed,
            processor: &stone_oven_burning_carbon,
            process: &burn_crushed_coal_to_coke,
        },
        Group {
            quantity: 10.0 / 1.0 * coke_to_carbon.time / liquifier_mk1.speed,
            processor: &liquifier_mk1,
            process: &coke_to_carbon,
        },
        Group {
            quantity: 24.0 / (1.0/Material::Carbon.joules()) * carbon_to_steam.time / boiler_mk1.speed,
            processor: &boiler_mk1,
            process: &carbon_to_steam,
        },
    ];

    // println!("{:#?}", &groups);

    let balance = accumulate_groups(&groups);
    println!("Material balance per second: {:#?}", balance);
}

#[derive(Debug)]
struct Group<'a> {
    quantity: f64,
    processor: &'a Processor,
    process: &'a Process,
}

fn accumulate_groups(groups: &Vec<Group>) -> std::collections::HashMap<Material, (f64, f64)> {
    let mut map = std::collections::HashMap::<Material, (f64, f64)>::new();

    for group in groups {
        let quantity = group.quantity;
        let processor = group.processor;
        let process = group.process;

        for ingredient in process.ingredients.iter() {
            let accumulator = map.entry(ingredient.material).or_insert((0.0, 0.0));
            let quantity_per_second = (ingredient.quantity / process.time) * (quantity * processor.speed);
            if quantity_per_second >= 0.0 {
                (*accumulator).0 += quantity_per_second;
            } else {
                (*accumulator).1 += quantity_per_second;
            }
        }

        assert!(processor.energy_source.joules() > 0.0);

        let quantity_per_second = quantity *
            -(processor.energy_consumption + processor.drain) /
            processor.energy_source.joules();

        let accumulator = map.entry(processor.energy_source).or_insert((0.0, 0.0));

        if quantity_per_second >= 0.0 {
            (*accumulator).0 += quantity_per_second;
        } else {
            (*accumulator).1 += quantity_per_second;
        }
    }

    map
}
