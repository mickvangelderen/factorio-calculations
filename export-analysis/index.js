const fs = require('fs');
const snake_case = require('snake-case');

function read_json_file(path) {
    return JSON.parse(fs.readFileSync(path, 'utf8'));
}

const recipes = read_json_file('recipes.json');
const entities = read_json_file('entities.json');
const items = read_json_file('items.json');

/*
enum Ingredient {
    Item {
        name: String,
        amount: u32,
    },
    Fluid {
        name: String,
        amount: u32,
    },
    FluidWithTemp {
        name: String,
        amount: u32,
        min_temp: u32,
        max_temp: u32,
    },
}

enum Product {
    Item {
        name: String,
        amount: u32,
    },
    Fluid {
        name: String,
        amount: u32,
    },
}
*/

function process_ingredient(ingredient) {
    if (ingredient.type == "item") {

    } else if (ingredient.type == "fluid") {

    } else {
        console.log(ingredient);
        throw new Error("Weird ingredient");
    }
}
    return {
        type: ingredient.type,
        name: snake_case(ingredient.name),
        amount: ingredient.amount,
    };
}

let r = Object.keys(recipes).map(id => {
    let recipe = recipes[id];
    return {
        name: snake_case(recipe.name),
        ingredients: recipe.ingredients.map(ingredient => {
        }).concat(
            recipe.products.map(product => {
                return {

                };
            })
        ),
    };
});

// "ingredients": [
//     {
//         "type": "item",
//         "name": "sulfur",
//         "amount": 5
//     },
//     {
//         "type": "item",
//         "name": "iron-plate",
//         "amount": 1
//     },
//     {
//         "type": "fluid",
//         "name": "water-purified",
//         "amount": 100
//     }
// ],
// "products": [
//     {
//         "type": "fluid",
//         "name": "liquid-sulfuric-acid",
//         "amount_min": 50,
//         "amount_max": 50,
//         "probability": 1
//     }
// ],
console.log(r);

// pub static dirt_water_electrolysis: Process = Process {
//     name: "dirt_water_electrolysis",
//     ingredients: &[
//         Ingredient {
//             material: Material::Water,
//             quantity: -100.0,
//         },
//         Ingredient {
//             material: Material::Oxygen,
//             quantity: 30.0,
//         },
//         Ingredient {
//             material: Material::Hydrogen,
//             quantity: 40.0,
//         },
//         Ingredient {
//             material: Material::Slag,
//             quantity: 1.0,
//         },
//     ],
//     time: 2.0,
// };
