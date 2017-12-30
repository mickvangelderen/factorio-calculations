#![allow(unused_variables)]

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Material {
    Water,
    MineralizedWater,
    CrushedStone,
    GreenAlgae,
    Fiber,
    WoodPellet,
    WoodBlock,
    Coal,
    Coke,
    Carbon,
    Slag,
    Hydrogen,
    Oxygen,
    CarbonDioxide,
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
}

fn main() {
    // Processors.
    let assembly_machine_mk1 = Processor {
        speed: 0.5,
        energy_consumption: 100_000.0,
    };

    let flare_stack = Processor {
        speed: 2.0,
        energy_consumption: 30_000.0,
    };

    let algae_farm_mk1 = Processor {
        speed: 1.0,
        energy_consumption: 120_000.0,
    };

    let electrolyser_mk1 = Processor {
        speed: 1.0,
        energy_consumption: 300_000.0,
    };

    let liquifier_mk1 = Processor {
        speed: 1.5,
        energy_consumption: 125_000.0,
    };

    let ore_crusher_mk1 = Processor {
        speed: 1.5,
        energy_consumption: 100_000.0,
    };

    let stone_oven = Processor {
        speed: 1.0,
        energy_consumption: 180_000.0,
    };

    let offshore_pump = Processor {
        speed: 1200.0,
        energy_consumption: 0.0,
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

    let wood_pellet_to_wood_block = Process {
        ingredients: vec![
            Ingredient {
                material: Material::WoodPellet,
                quantity: -8.0,
            },
            Ingredient {
                material: Material::WoodBlock,
                quantity: 4.0,
            },
        ],
        time: 2.0,
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
            quantity: 675.0/(flare_stack.speed*100.0),
            processor: &flare_stack,
            process: &burn_oxygen,
        },
        Group {
            quantity: 900.0/(flare_stack.speed*100.0),
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
            quantity: 180.0/(liquifier_mk1.speed/green_algae_to_fiber.time*10.0),
            processor: &liquifier_mk1,
            process: &green_algae_to_fiber,
        },
        Group {
            quantity: 90.0/(assembly_machine_mk1.speed/fiber_to_wood_pellet.time*12.0),
            processor: &assembly_machine_mk1,
            process: &fiber_to_wood_pellet,
        },
    ];

    println!("{:#?}", &groups);

    let (map, energy_consumption) = accumulate_groups(&groups);
    println!("Material balance: {:#?}", map);
    println!("Energy consumption: {:?} Watt", energy_consumption);
}

#[derive(Debug)]
struct Group<'a> {
    quantity: f64,
    processor: &'a Processor,
    process: &'a Process,
}

fn accumulate_groups(groups: &Vec<Group>) -> (std::collections::HashMap<Material, f64>, f64) {
    let mut map = std::collections::HashMap::<Material, f64>::new();
    let mut energy_consumption = 0.0f64;

    for group in groups {
        let quantity = group.quantity;
        let processor = group.processor;
        let process = group.process;

        for ingredient in process.ingredients.iter() {
            let accumulator = map.entry(ingredient.material).or_insert(0.0);
            *accumulator += (ingredient.quantity/process.time)*(quantity*processor.speed);
        }
        energy_consumption += quantity*processor.energy_consumption;
    }

    (map, energy_consumption)
}
