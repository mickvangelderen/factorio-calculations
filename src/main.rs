#![allow(unused)]

extern crate rulinalg;

use rulinalg::matrix::*;
use rulinalg::vector::*;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Material {
    Carbon,
    CarbonDioxide,
    Coal,
    Coke,
    CokePellet,
    ConcentratedMudWater,
    CrushedCoal,
    CrushedStone,
    Fiber,
    GreenAlgae,
    HeavyMudWater,
    Hydrogen,
    Joule,
    MineralizedWater,
    Mud,
    Nodule,
    Oxygen,
    Slag,
    ViscousMudWater,
    Water,
    WoodBrick,
    WoodPellet,
}

impl Material {
    fn joules(&self) -> f64 {
        use Material::*;

        match *self {
            Carbon => 6_000_000.0,
            Coal => 8_000_000.0,
            Coke => 5_000_000.0,
            CokePellet => 30_000_000.0,
            Fiber => 1_000_000.0,
            Joule => 1.0,
            WoodPellet => 12_000_000.0,
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
    name: String,
    ingredients: Vec<Ingredient>,
    time: f64,
}

#[derive(Debug)]
pub struct Processor {
    name: String,
    speed: f64,
    energy_consumption: f64,
    energy_source: Material,
    drain: f64,
}

fn main() {
    // early_energy();
    nodules();
}

fn early_energy() {
    // Processors.
    let boiler_mk1_burning_carbon = Processor {
        name: String::from("boiler_mk1_burning_carbon"),
        speed: 1.0,
        energy_consumption: 3_600_000.0,
        energy_source: Material::Carbon,
        drain: 0.0,
    };

    let boiler_mk2_burning_carbon = Processor {
        name: String::from("boiler_mk2_burning_carbon"),
        speed: 1.0,
        energy_consumption: 3_600_000.0,
        energy_source: Material::Carbon,
        drain: 0.0,
    };

    let assembly_machine_mk1 = Processor {
        name: String::from("assembly_machine_mk1"),
        speed: 0.5,
        energy_consumption: 100_000.0,
        energy_source: Material::Joule,
        drain: 3_300.0,
    };

    let flare_stack = Processor {
        name: String::from("flare_stack"),
        speed: 2.0,
        energy_consumption: 30_000.0,
        energy_source: Material::Joule,
        drain: 1_000.0,
    };

    let algae_farm_mk1 = Processor {
        name: String::from("algae_farm_mk1"),
        speed: 1.0,
        energy_consumption: 120_000.0,
        energy_source: Material::Joule,
        drain: 4_000.0,
    };

    let electrolyser_mk1 = Processor {
        name: String::from("electrolyser_mk1"),
        speed: 1.0,
        energy_consumption: 300_000.0,
        energy_source: Material::Joule,
        drain: 10_000.0,
    };

    let liquifier_mk1 = Processor {
        name: String::from("liquifier_mk1"),
        speed: 1.5,
        energy_consumption: 125_000.0,
        energy_source: Material::Joule,
        drain: 4_100.0,
    };

    let ore_crusher_mk1 = Processor {
        name: String::from("ore_crusher_mk1"),
        speed: 1.5,
        energy_consumption: 100_000.0,
        energy_source: Material::Joule,
        drain: 3_300.0,
    };

    let stone_oven_burning_carbon = Processor {
        name: String::from("stone_oven_burning_carbon"),
        speed: 1.0,
        energy_consumption: 180_000.0,
        energy_source: Material::Carbon,
        drain: 0.0,
    };

    let offshore_pump = Processor {
        name: String::from("offshor_pump"),
        speed: 1200.0,
        energy_consumption: 0.0,
        energy_source: Material::Joule,
        drain: 0.0,
    };

    // Processes.
    let water_pumping = Process {
        name: String::from("water_pumping"),
        ingredients: vec![
            Ingredient {
                material: Material::Water,
                quantity: 1.0,
            },
        ],
        time: 1.0,
    };

    let dirt_water_electrolysis = Process {
        name: String::from("dirt_water_electrolysis"),
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
        name: String::from("stone_crushing"),
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
        name: String::from("water_mineralization"),
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
        name: String::from("green_algae_growing"),
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
        name: String::from("green_algae_to_fiber"),
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
        name: String::from("fiber_to_wood_pellet"),
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
        name: String::from("wood_pellet_to_wood_brick"),
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
        name: String::from("wood_brick_to_coal"),
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
        name: String::from("coal_to_carbon_dioxide"),
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
        name: String::from("coal_to_crushed_coal"),
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
        name: String::from("burn_crushed_coal_to_coke"),
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
        name: String::from("coke_to_carbon"),
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

    let boiler_mk1_carbon_to_power = Process {
        name: String::from("boiler_mk1_carbon_to_power"),
        // 50% efficiency.
        ingredients: vec![
            Ingredient {
                material: Material::Joule,
                quantity: 0.5 * boiler_mk1_burning_carbon.energy_consumption,
            },
        ],
        time: 1.0,
    };

    let boiler_mk2_carbon_to_power = Process {
        name: String::from("boiler_mk2_carbon_to_power"),
        // 60% efficiency.
        ingredients: vec![
            Ingredient {
                material: Material::Joule,
                quantity: 0.6 * boiler_mk2_burning_carbon.energy_consumption,
            },
        ],
        time: 1.0,
    };

    let burn_oxygen = Process {
        name: String::from("burn_oxygen"),
        ingredients: vec![
            Ingredient {
                material: Material::Oxygen,
                quantity: -100.0,
            },
        ],
        time: 1.0,
    };

    let burn_hydrogen = Process {
        name: String::from("burn_hydrogen"),
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
            quantity: 28.0 / (3.6 /* J/s/boiler */ / 6.0 /* J/carbon */) *
                boiler_mk1_carbon_to_power.time /
                boiler_mk1_burning_carbon.speed,
            processor: &boiler_mk1_burning_carbon,
            process: &boiler_mk1_carbon_to_power,
        },
    ];

    for group in &groups {
        println!(
            "{} x {} performing {:#?}",
            group.quantity,
            group.processor.name,
            group.process
        );
    }

    // println!("{:#?}", &groups);

    let balance = accumulate_groups(&groups);
    println!(
        "Material (production, consumption) per second: {:#?}",
        balance
    );

}

#[derive(Debug)]
struct Group<'a> {
    quantity: f64,
    processor: &'a Processor,
    process: &'a Process,
}

fn accumulate_groups(groups: &Vec<Group>) -> std::collections::BTreeMap<Material, (f64, f64)> {
    let mut map = std::collections::BTreeMap::<Material, (f64, f64)>::new();

    for group in groups {
        let quantity = group.quantity;
        let processor = group.processor;
        let process = group.process;

        for ingredient in process.ingredients.iter() {
            let accumulator = map.entry(ingredient.material).or_insert((0.0, 0.0));
            let quantity_per_second = (ingredient.quantity / process.time) *
                (quantity * processor.speed);
            if quantity_per_second >= 0.0 {
                (*accumulator).0 += quantity_per_second;
            } else {
                (*accumulator).1 += quantity_per_second;
            }
        }

        assert!(processor.energy_source.joules() > 0.0);

        let quantity_per_second = quantity * -(processor.energy_consumption + processor.drain) /
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

// use std::io::prelude::*;

fn nodules() {
    // Processors.
    let seafloor_pump = Processor {
        name: String::from("seafloor_pump"),
        speed: 300.0,
        energy_consumption: 0.0,
        energy_source: Material::Joule,
        drain: 0.0,
    };

    let washing_plant_mk1 = Processor {
        name: String::from("washing_plant_mk1"),
        speed: 1.5,
        energy_consumption: 100_000.0,
        energy_source: Material::Joule,
        drain: 3_300.0,
    };

    // Processes.
    let pump_viscous_mud_water = Process {
        name: String::from("pump_viscous_mud_water"),
        ingredients: vec![
            Ingredient {
                material: Material::ViscousMudWater,
                quantity: 1.0,
            },
        ],
        time: 1.0,
    };

    let wash_viscous_mud_water = Process {
        name: String::from("wash_viscous_mud_water"),
        ingredients: vec![
            Ingredient {
                material: Material::ViscousMudWater,
                quantity: -100.0,
            },
            Ingredient {
                material: Material::Water,
                quantity: -50.0,
            },
            Ingredient {
                material: Material::HeavyMudWater,
                quantity: 100.0,
            },
            Ingredient {
                material: Material::Mud,
                quantity: 1.0,
            },
        ],
        time: 5.0,
    };

    let wash_heavy_mud_water = Process {
        name: String::from("wash_heavy_mud_water"),
        ingredients: vec![
            Ingredient {
                material: Material::HeavyMudWater,
                quantity: -100.0,
            },
            Ingredient {
                material: Material::Water,
                quantity: -50.0,
            },
            Ingredient {
                material: Material::ConcentratedMudWater,
                quantity: 100.0,
            },
            Ingredient {
                material: Material::Mud,
                quantity: 1.0,
            },
        ],
        time: 5.0,
    };

    let heavy_mud_water_to_nodule = Process {
        name: String::from("heavy_mud_water_to_nodule"),
        ingredients: vec![
            Ingredient {
                material: Material::HeavyMudWater,
                quantity: -40.0,
            },
            Ingredient {
                material: Material::Water,
                quantity: -25.0,
            },
            Ingredient {
                material: Material::Nodule,
                quantity: 1.0,
            },
        ],
        time: 5.0,
    };

    struct Group<'a> {
        quantity: Option<f64>,
        processor: &'a Processor,
        process: &'a Process,
    };

    // Setup.
    let groups = vec![
        Group {
            quantity: Some(1.0),
            processor: &seafloor_pump,
            process: &pump_viscous_mud_water,
        },
        Group {
            quantity: None,
            processor: &washing_plant_mk1,
            process: &wash_viscous_mud_water,
        },
        Group {
            quantity: None,
            processor: &washing_plant_mk1,
            process: &heavy_mud_water_to_nodule,
        },
    ];

    let mut material_to_row = std::collections::BTreeMap::<Material, usize>::new();

    for group in &groups {
        for ingredient in &group.process.ingredients {
            material_to_row.entry(ingredient.material).or_insert(0);
        }
        material_to_row
            .entry(group.processor.energy_source)
            .or_insert(0);
    }

    for (index, (material, row)) in material_to_row.iter_mut().enumerate() {
        *row = index
    }

    let fixed_processes: Vec<(usize, f64)> = groups.iter().enumerate().filter_map(|(i, g)| g.quantity.map(|q| (i, q))).collect();

    let fixed_materials = vec![
        (Material::HeavyMudWater, 0.0),
        (Material::ViscousMudWater, 0.0),
    ];

    // Recipe matrix.
    #[allow(non_snake_case)]
    let mut R: Matrix<f64> = Matrix::zeros(material_to_row.len(), groups.len());

    for (col, group) in groups.iter().enumerate() {
        for ingredient in &group.process.ingredients {
            let row = *material_to_row.get(&ingredient.material).unwrap();
            R[[row, col]] = group.processor.speed * ingredient.quantity / group.process.time;
        }
        let row = *material_to_row
            .get(&group.processor.energy_source)
            .unwrap();
        R[[row, col]] = group.processor.energy_consumption + group.processor.drain;
    }

    #[allow(non_snake_case)]
    let I: Matrix<f64> = -Matrix::identity(material_to_row.len());

    #[allow(non_snake_case)]
    let Rv: Vector<f64> = Vector::zeros(material_to_row.len());

    // Process constraint matrices.

    #[allow(non_snake_case)]
    let mut P: Matrix<f64> = Matrix::zeros(fixed_processes.len(), groups.len());

    #[allow(non_snake_case)]
    let P0: Matrix<f64> = Matrix::zeros(fixed_processes.len(), material_to_row.len());

    #[allow(non_snake_case)]
    let mut Pv: Vector<f64> = Vector::zeros(fixed_processes.len());

    for (row, &(col, q)) in fixed_processes.iter().enumerate() {
        P[[row, col]] = 1.0;
        Pv[row] = q;
    }

    // Material constraint matrices.

    #[allow(non_snake_case)]
    let M0: Matrix<f64> = Matrix::zeros(fixed_materials.len(), groups.len());

    #[allow(non_snake_case)]
    let mut M: Matrix<f64> = Matrix::zeros(fixed_materials.len(), material_to_row.len());

    #[allow(non_snake_case)]
    let mut Mv: Vector<f64> = Vector::zeros(fixed_materials.len());

    for (row, &(mat, q)) in fixed_materials.iter().enumerate() {
        let col = material_to_row[&mat];
        M[[row, col]] = 1.0;
        Mv[row] = q;
    }

    let a: Matrix<f64> =
        (&R.hcat(&I))
        .vcat(&P.hcat(&P0))
        .vcat(&M0.hcat(&M))
        ;

    use std::iter::FromIterator;

    let b: Vector<f64> = Vector::from_iter(Rv.into_iter().chain(Pv.into_iter()).chain(Mv.into_iter()));

    let x = a.solve(b).unwrap();

    println!("Setup");
    for (i, group) in groups.iter().enumerate() {
        println!("{:.2} x {} performing {}", x[i], group.processor.name, group.process.name);
    }

    println!();
    println!("Balance");
    for (i, (material, _)) in material_to_row.iter().enumerate() {
        println!("{:.2} {:?}/s", x[groups.len() + i], material);
    }
}
