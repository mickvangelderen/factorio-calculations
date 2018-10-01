const fs = require('fs');
const snake_case = require('snake-case');
const constant_case = require('constant-case');

function read_json_file(path) {
    return JSON.parse(fs.readFileSync(path, 'utf8'));
}

function isNumber(value) {
    return typeof value === "number";
}

function printFloat(value) {
    return Number.parseFloat(value).toPrecision(21);
}

const items = read_json_file('items.json');
const fluids = read_json_file('fluids.json');
const entities = read_json_file('entities.json');
const recipes = read_json_file('recipes.json');

// https://lua-api.factorio.com/0.16.51/LuaItemPrototype.html
const items_rust = Object.values(items).map(item => {
    const { name, fuel_value } = item;
    return `
static ${constant_case(name)}: Item = Item {
    name: "${name}",
    fuel_value: ${printFloat(fuel_value)},
}
`;
}).join('');

// https://lua-api.factorio.com/0.16.51/LuaFluidPrototype.html
const fluids_rust = Object.values(fluids).map(fluid => {
    const { name, fuel_value } = fluid;
    return `
static ${constant_case(name)}: Fluid = Fluid {
    name: "${name}",
    fuel_value: ${printFloat(fuel_value)},
}
`;
}).join('');

// https://lua-api.factorio.com/0.16.51/LuaEntityPrototype.html
const entities_rust = Object.values(entities).map(entity => {
    const { name, crafting_speed } = entity;
    return `
static ${constant_case(name)}: Entity = Entity {
    name: "${name}",
    crafting_speed: ${isNumber(crafting_speed) ? `Some(${printFloat(crafting_speed)})` : 'None'},
}
`;
}).join('');

// type :: string: "item" or "fluid".
// name :: string: Prototype name of the required item or fluid.
// amount :: uint: Amount of the item.
// minimum_temperature :: uint (optional): The minimum fluid temperature required. Has no effect if type is '"item"'.
// maximum_temperature :: uint (optional): The maximum fluid temperature allowed. Has no effect if type is '"item"'.
function ingredient_to_rust(ingredient) {
    const { name, amount, type } = ingredient;
    if (isNumber(amount)) {
        if (type === "item") {
                return `
        Ingredient::Item {
            name: "${name}",
            amount: ${amount},
        },`;
        }
        if (type == "fluid") {
            const { minimum_temperature, maximum_temperature } = ingredient;
            if (isNumber(minimum_temperature) && isNumber(maximum_temperature)) {
                return `
        Ingredient::FluidTemp {
            name: "${name}",
            amount: ${amount},
            minimum_temperature: ${minimum_temperature},
            maximum_temperature: ${maximum_temperature},
        },`;
            } else {
                return `
        Ingredient::Fluid {
            name: "${name}",
            amount: ${amount},
        },`;
            }
        }
    }
    console.error(ingredient);
    throw new Error("Invalid ingredient.");
}

// type :: string: "item" or "fluid".
// name :: string: Prototype name of the result.
// amount :: float (optional): Amount of the item or fluid to give. If not specified, amount_min, amount_max and probability must all be specified.
// temperature :: uint (optional): The fluid temperature of this product. Has no effect if type is '"item"'.
// amount_min :: uint (optional): Minimal amount of the item or fluid to give. Has no effect when amount is specified.
// amount_max :: uint (optional): Maximum amount of the item or fluid to give. Has no effect when amount is specified.
// probability :: double (optional): A value in range [0, 1]. Item or fluid is only given with this probability; otherwise no product is produced. Has no effect when amount is specified.
function product_to_rust(product) {
    if (product.type === "item") {
        const { name, amount } = product;
        if (isNumber(amount)) {
            return `
        Product::Item {
            name: "${name}",
            amount: ${amount},
        },`;
        }
        const { amount_min, amount_max, probability } = product;
        if (isNumber(amount_min) && isNumber(amount_max) && isNumber(probability)) {
            return `
        Product::ItemChance {
            name: "${name}",
            amount_min: ${amount_min},
            amount_max: ${amount_max},
            probability: ${probability},
        },`;
        }
    } else if (product.type === "fluid") {
        const { name, amount } = product;
        if (isNumber(amount)) {
            const { temperature } = product;
            if (isNumber(temperature)) {
                return `
        Product::FluidTemp {
            name: "${name}",
            amount: ${amount},
            temperature: ${temperature},
        },`;
            } else {
                return `
        Product::Fluid {
            name: "${name}",
            amount: ${amount},
        },`;
            }
        }
        const { amount_min, amount_max, probability } = product;
        if (isNumber(amount_min) && isNumber(amount_max) && isNumber(probability)) {
            const { temperature } = product;
            if (isNumber(temperature)) {
                return `
        Product::FluidChanceTemp {
            name: "${name}",
            amount_min: ${amount_min},
            amount_max: ${amount_max},
            probability: ${probability},
            temperature: ${temperature},
        },`;
            } else {
                return `
        Product::FluidChance {
            name: "${name}",
            amount_min: ${amount_min},
            amount_max: ${amount_max},
            probability: ${probability},
        },`;
            }
        }
    }
    console.error(product);
    throw new Error("Invalid product.");
}

const recipes_rust = Object.values(recipes).map(recipe => {
    return `
static ${constant_case(recipe.name)}: Recipe = Recipe {
    name: "${recipe.name}",
    ingredients: [${recipe.ingredients.map(ingredient_to_rust).join('')}
    ],
    products: [${recipe.products.map(product_to_rust).join('')}
    ],
    time: ${recipe.energy_required}
};
`;
}).join('');

fs.writeFileSync('items.rs', items_rust);
fs.writeFileSync('fluids.rs', fluids_rust);
fs.writeFileSync('entities.rs', entities_rust);
fs.writeFileSync('recipes.rs', recipes_rust);
