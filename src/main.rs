#![allow(unused)]
#![allow(non_upper_case_globals)]

mod items;
mod entities;
mod recipes;

use rulinalg::matrix::*;
use rulinalg::vector::*;
use items::*;
use entities::*;
use recipes::*;
use std::collections::HashMap;

fn main() {
    reinforced_iron_plates();
}

#[derive(Debug)]
struct Group<'a> {
    quantity: Option<f64>,
    processor: &'a Entity,
    process: &'a Recipe,
}

#[derive(Debug)]
struct FixedGroup<'a> {
    quantity: f64,
    processor: &'a Entity,
    process: &'a Recipe,
}

#[derive(Debug)]
struct FixedItem {
    item: &'static Item,
    quantity_per_minute: f64,
}

fn reinforced_iron_plates() {
    let groups = &[
        Group {
            quantity: None,
            processor: &CONSTRUCTOR,
            process: &CRAFT_IRON_ROD,
        },
        Group {
            quantity: None,
            processor: &CONSTRUCTOR,
            process: &CRAFT_SCREW,
        },
        Group {
            quantity: Some(3.0),
            processor: &ASSEMBLER,
            process: &ASSEMBLE_REINFORCED_IRON_PLATE,
        }
    ];

    let fixed_items = &[
        FixedItem { item: &IRON_ROD, quantity_per_minute: 0.0 },
        FixedItem { item: &SCREW, quantity_per_minute: 0.0 },
    ];

    solve_and_print(groups, fixed_items);
}

fn solve_and_print(groups: &[Group], fixed_items: &[FixedItem]) {
    let fixed_processes: Vec<(usize, f64)> = groups
        .iter()
        .enumerate()
        .filter_map(|(i, g)| g.quantity.map(|q| (i, q)))
        .collect();

    let item_to_row = {
        let mut item_to_row: HashMap<*const Item, usize> = HashMap::new();

        for group in groups.iter() {
            for ingredient in group.process.ingredients.iter() {
                let row = item_to_row.len();
                item_to_row.entry(ingredient.item).or_insert(row);
            }

            for product in group.process.products.iter() {
                let row = item_to_row.len();
                item_to_row.entry(product.item).or_insert(row);
            }
        }

        item_to_row
    };

    // Recipe matrix.
    #[allow(non_snake_case)]
    let mut R: Matrix<f64> = Matrix::zeros(item_to_row.len(), groups.len());

    println!("{:#?}", groups);
    println!("{:#?}", fixed_processes);
    println!("{:#?}", item_to_row);
    println!("{:#?}", R);
    for (col, group) in groups.iter().enumerate() {
        let Group { process, processor, quantity } = group;
        let crafts_per_second = processor.crafting_speed / process.time;
        for ingredient in process.ingredients {
            let row = *item_to_row.get(&(ingredient.item as *const _)).unwrap();
            R[[row, col]] -= crafts_per_second * ingredient.quantity;
        }
        for product in process.products {
            let row = *item_to_row.get(&(product.item as *const _)).unwrap();
            R[[row, col]] += crafts_per_second * product.quantity;
        }
    }

    #[allow(non_snake_case)]
    let I: Matrix<f64> = -Matrix::identity(item_to_row.len());

    #[allow(non_snake_case)]
    let Rv: Vector<f64> = Vector::zeros(item_to_row.len());

    // Process constraint matrices.

    #[allow(non_snake_case)]
    let mut P: Matrix<f64> = Matrix::zeros(fixed_processes.len(), groups.len());

    #[allow(non_snake_case)]
    let P0: Matrix<f64> = Matrix::zeros(fixed_processes.len(), item_to_row.len());

    #[allow(non_snake_case)]
    let mut Pv: Vector<f64> = Vector::zeros(fixed_processes.len());

    for (row, &(col, q)) in fixed_processes.iter().enumerate() {
        P[[row, col]] = 1.0;
        Pv[row] = q;
    }

    // Item constraint matrices.

    #[allow(non_snake_case)]
    let M0: Matrix<f64> = Matrix::zeros(fixed_items.len(), groups.len());

    #[allow(non_snake_case)]
    let mut M: Matrix<f64> = Matrix::zeros(fixed_items.len(), item_to_row.len());

    #[allow(non_snake_case)]
    let mut Mv: Vector<f64> = Vector::zeros(fixed_items.len());

    for (row, fixed) in fixed_items.iter().enumerate() {
        let col = *item_to_row.get(&(fixed.item as *const _)).expect(&format!(
            "{:?} does not participate in any of the recipes or as fuel",
            fixed.item
        ));
        M[[row, col]] = 1.0;
        Mv[row] = fixed.quantity_per_minute / 60.0;
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
    for (i, (&item_ptr, _)) in item_to_row.iter().enumerate() {
        let item: &'static Item = unsafe { &*item_ptr };
        println!("{:>12.2} {}/m", x[groups.len() + i]*60.0, item.name);
    }

    let groups: Vec<FixedGroup> = groups.iter().enumerate().map(|(i, group)| {
        FixedGroup {
            quantity: x[i],
            process: group.process,
            processor: group.processor,
        }
    }).collect();

    println!();
    println!(" Consumption  Production");
    let cons_prod = calculate_consumption_production(&groups);
    for (item_ptr, (cons, prod)) in cons_prod.into_iter() {
        let item: &'static Item = unsafe { &*item_ptr };
        println!("{:>12.2} {:>12.2} {}/m", cons*60.0, prod*60.0, item.name);
    }

    println!();
}

fn calculate_consumption_production(groups: &[FixedGroup]) -> HashMap<*const Item, (f64, f64)> {
    let mut map: HashMap<*const Item, (f64, f64)> = HashMap::new();

    for group in groups {
        let FixedGroup { processor, process, quantity } = group;

        let crafts_per_second = quantity * processor.crafting_speed / process.time;

        for ingredient in process.ingredients.iter() {
            let accumulator = map.entry(ingredient.item).or_insert((0.0, 0.0));
            (*accumulator).0 += crafts_per_second * ingredient.quantity;
        }

        for product in process.products.iter() {
            let accumulator = map.entry(product.item).or_insert((0.0, 0.0));
            (*accumulator).1 += crafts_per_second * product.quantity;
        }
    }

    map
}
