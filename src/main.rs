#![allow(unused)]
#![allow(non_upper_case_globals)]

extern crate rulinalg;

use rulinalg::matrix::*;
use rulinalg::vector::*;

mod processors;
mod processes;

use processors::*;
use processes::*;

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
    PurifiedWater,
    SalineWater,
    Slag,
    Sulfur,
    SulfuricWasteWater,
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
    name: &'static str,
    ingredients: &'static [Ingredient],
    time: f64,
}

#[derive(Debug)]
pub struct Processor {
    name: &'static str,
    speed: f64,
    energy_consumption: f64,
    energy_source: Material,
    drain: f64,
}

fn main() {
    easy_early_energy();
    // early_energy();
    // advanced_early_energy();
    // nodules();
}

fn easy_early_energy() {
    // Do things.
    let groups = vec![
        Group {
            quantity: None,
            processor: &offshore_pump,
            process: &water_pumping,
        },
        Group {
            quantity: None,
            processor: &flare_stack,
            process: &burn_oxygen,
        },
        Group {
            quantity: None,
            processor: &flare_stack,
            process: &burn_hydrogen,
        },
        Group {
            quantity: Some(1.0),
            processor: &electrolyser_mk1,
            process: &dirt_water_electrolysis,
        },
        Group {
            quantity: None,
            processor: &ore_crusher_mk1,
            process: &stone_crushing,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &water_mineralization,
        },
        Group {
            quantity: None,
            processor: &algae_farm_mk1,
            process: &green_algae_growing,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &green_algae_to_fiber,
        },
        Group {
            quantity: None,
            processor: &assembly_machine_mk1,
            process: &fiber_to_wood_pellet,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &wood_pellet_to_carbon_dioxide,
        },
        Group {
            quantity: None,
            processor: &boiler_mk1_burning_wood_pellet,
            process: &boiler_mk1_power,
        },
    ];

    solve_and_print(
        groups,
        vec![
            (Material::CarbonDioxide, 0.0),
            (Material::CrushedStone, 0.0),
            (Material::Fiber, 0.0),
            (Material::GreenAlgae, 0.0),
            (Material::Hydrogen, 0.0),
            // (Material::Joule, 0.0),
            (Material::MineralizedWater, 0.0),
            (Material::Oxygen, 0.0),
            (Material::Slag, 0.0),
            (Material::Water, 0.0),
            (Material::WoodPellet, 0.0),
        ],
    );
}

fn early_energy() {
    let groups = vec![
        Group {
            quantity: None,
            processor: &offshore_pump,
            process: &water_pumping,
        },
        Group {
            quantity: None,
            processor: &flare_stack,
            process: &burn_oxygen,
        },
        Group {
            quantity: None,
            processor: &flare_stack,
            process: &burn_hydrogen,
        },
        Group {
            quantity: None,
            processor: &electrolyser_mk1,
            process: &dirt_water_electrolysis,
        },
        Group {
            quantity: None,
            processor: &ore_crusher_mk1,
            process: &stone_crushing,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &water_mineralization,
        },
        Group {
            quantity: None,
            processor: &algae_farm_mk1,
            process: &green_algae_growing,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &green_algae_to_fiber,
        },
        Group {
            quantity: None,
            processor: &assembly_machine_mk1,
            process: &fiber_to_wood_pellet,
        },
        Group {
            quantity: None,
            processor: &assembly_machine_mk1,
            process: &wood_pellet_to_wood_brick,
        },
        Group {
            quantity: None,
            processor: &stone_furnace_burning_carbon,
            process: &wood_brick_to_coal,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &coal_to_carbon_dioxide,
        },
        Group {
            quantity: Some(1.0),
            processor: &ore_crusher_mk1,
            process: &coal_to_crushed_coal,
        },
        Group {
            quantity: None,
            processor: &stone_furnace_burning_carbon,
            process: &burn_crushed_coal_to_coke,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &coke_to_carbon,
        },
        Group {
            quantity: None,
            processor: &boiler_mk1_burning_carbon,
            process: &boiler_mk1_power,
        },
    ];

    solve_and_print(
        groups,
        vec![
            (Material::Carbon, 0.0),
            (Material::CarbonDioxide, 0.0),
            (Material::Coal, 0.0),
            (Material::Coke, 0.0),
            (Material::CrushedCoal, 0.0),
            (Material::CrushedStone, 0.0),
            (Material::Fiber, 0.0),
            (Material::GreenAlgae, 0.0),
            (Material::Hydrogen, 0.0),
            // (Material::Joule, 0.0),
            (Material::MineralizedWater, 0.0),
            (Material::Oxygen, 0.0),
            (Material::Slag, 0.0),
            (Material::Water, 0.0),
            (Material::WoodBrick, 0.0),
            (Material::WoodPellet, 0.0),
        ],
    );
}

fn advanced_early_energy() {
    let groups = vec![
        Group {
            quantity: None,
            processor: &offshore_pump,
            process: &water_pumping,
        },
        Group {
            quantity: None,
            processor: &flare_stack,
            process: &burn_oxygen,
        },
        Group {
            quantity: None,
            processor: &flare_stack,
            process: &burn_hydrogen,
        },
        Group {
            quantity: None,
            processor: &electrolyser_mk1,
            process: &dirt_water_electrolysis,
        },
        Group {
            quantity: None,
            processor: &ore_crusher_mk1,
            process: &stone_crushing,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &water_mineralization,
        },
        Group {
            quantity: Some(10.0),
            processor: &algae_farm_mk1,
            process: &green_algae_growing,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &green_algae_to_fiber,
        },
        Group {
            quantity: None,
            processor: &assembly_machine_mk1,
            process: &fiber_to_wood_pellet,
        },
        Group {
            quantity: None,
            processor: &assembly_machine_mk1,
            process: &wood_pellet_to_wood_brick,
        },
        Group {
            quantity: None,
            processor: &stone_furnace_burning_carbon,
            process: &wood_brick_to_coal,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &coal_to_carbon_dioxide,
        },
        Group {
            quantity: None,
            processor: &ore_crusher_mk1,
            process: &coal_to_crushed_coal,
        },
        Group {
            quantity: None,
            processor: &hydro_plant_mk1,
            process: &water_purification,
        },
        Group {
            quantity: None,
            processor: &clarifier,
            process: &void_saline_water,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &clean_coal_to_coke_and_sulfuric_waste_water,
        },
        Group {
            quantity: None,
            processor: &hydro_plant_mk1,
            process: &sulfuric_waste_water_purification,
        },
        Group {
            quantity: None,
            processor: &liquifier_mk1,
            process: &coke_to_carbon,
        },
        Group {
            quantity: None,
            processor: &boiler_mk1_burning_carbon,
            process: &boiler_mk1_power,
        },
    ];

    solve_and_print(
        groups,
        vec![
            (Material::Carbon, 0.0),
            (Material::CarbonDioxide, 0.0),
            (Material::Coal, 0.0),
            (Material::Coke, 0.0),
            (Material::CrushedCoal, 0.0),
            (Material::CrushedStone, 0.0),
            (Material::Fiber, 0.0),
            (Material::GreenAlgae, 0.0),
            (Material::Hydrogen, 0.0),
            // (Material::Joule, 0.0),
            (Material::MineralizedWater, 0.0),
            (Material::Oxygen, 0.0),
            (Material::PurifiedWater, 0.0),
            (Material::SalineWater, 0.0),
            (Material::Slag, 0.0),
            // (Material::Sulfur, 0.0),
            (Material::SulfuricWasteWater, 0.0),
            (Material::Water, 0.0),
            (Material::WoodBrick, 0.0),
            (Material::WoodPellet, 0.0),
        ],
    );
}

#[derive(Debug)]
struct FixedGroup<'a> {
    quantity: f64,
    processor: &'a Processor,
    process: &'a Process,
}

fn accumulate_groups(groups: &Vec<FixedGroup>) -> std::collections::BTreeMap<Material, (f64, f64)> {
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

fn nodules() {


    // Setup.
    let groups = vec![
        Group {
            quantity: None,
            processor: &offshore_pump,
            process: &water_pumping,
        },
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

    let fixed_materials = vec![
        (Material::Water, 0.0),
        (Material::HeavyMudWater, 0.0),
        (Material::ViscousMudWater, 0.0),
    ];

    solve_and_print(groups, fixed_materials);
}

struct Group<'a> {
    quantity: Option<f64>,
    processor: &'a Processor,
    process: &'a Process,
}

fn solve_and_print(groups: Vec<Group>, fixed_materials: Vec<(Material, f64)>) {
    let fixed_processes: Vec<(usize, f64)> = groups
        .iter()
        .enumerate()
        .filter_map(|(i, g)| g.quantity.map(|q| (i, q)))
        .collect();

    let mut material_to_row = std::collections::BTreeMap::<Material, usize>::new();

    for group in &groups {
        for ingredient in group.process.ingredients {
            material_to_row.entry(ingredient.material).or_insert(0);
        }
        material_to_row
            .entry(group.processor.energy_source)
            .or_insert(0);
    }

    for (index, (material, row)) in material_to_row.iter_mut().enumerate() {
        *row = index
    }

    // Recipe matrix.
    #[allow(non_snake_case)]
    let mut R: Matrix<f64> = Matrix::zeros(material_to_row.len(), groups.len());

    for (col, group) in groups.iter().enumerate() {
        for ingredient in group.process.ingredients {
            let row = *material_to_row.get(&ingredient.material).unwrap();
            R[[row, col]] = group.processor.speed * ingredient.quantity / group.process.time;
        }
        let row = *material_to_row.get(&group.processor.energy_source).unwrap();
        R[[row, col]] = -(group.processor.energy_consumption + group.processor.drain)/group.processor.energy_source.joules();
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
        let col = *material_to_row.get(&mat).expect(&format!(
            "{:?} does not participate in any of the recipes or as fuel",
            mat
        ));
        M[[row, col]] = 1.0;
        Mv[row] = q;
    }

    let a: Matrix<f64> = (&R.hcat(&I)).vcat(&P.hcat(&P0)).vcat(&M0.hcat(&M));

    use std::iter::FromIterator;

    let b: Vector<f64> =
        Vector::from_iter(Rv.into_iter().chain(Pv.into_iter()).chain(Mv.into_iter()));

    let x = a.solve(b).unwrap();

    println!("Setup");
    for (i, group) in groups.iter().enumerate() {
        println!(
            "{:>12.2} x {} ({})",
            x[i],
            group.process.name,
            group.processor.name
        );
    }

    println!();
    println!("Balance");
    for (i, (material, _)) in material_to_row.iter().enumerate() {
        println!("{:>12.2} {:?}/s", x[groups.len() + i], material);
    }

    let groups: Vec<FixedGroup> = groups.iter().enumerate().map(|(i, group)| {
        FixedGroup {
            quantity: x[i],
            process: group.process,
            processor: group.processor,
        }
    }).collect();

    println!();
    println!("  Production  Consumption");
    let prod_cons = accumulate_groups(&groups);
    for (mat, (prod, cons)) in prod_cons.into_iter() {
        println!("{:>12.2} {:>12.2} {:?}/m", prod*60.0, cons*60.0, mat);
    }

    println!();
}
