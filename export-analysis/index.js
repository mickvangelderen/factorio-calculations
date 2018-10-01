const fs = require('fs');
const snake_case = require('snake-case');
const constant_case = require('constant-case');

function read_json_file(path) {
    return JSON.parse(fs.readFileSync(path, 'utf8'));
}

function isNumber(value) {
    return typeof value === "number";
}

function isObject(value) {
    return typeof value === "object" && value !== null;
}

function printFloat(value) {
    return Number.parseFloat(value).toPrecision(21);
}

function printOptionFloat(value) {
    return isNumber(value) ? `Some(${printFloat(value)})` : 'None';
}

const items = read_json_file('items.json');
const fluids = read_json_file('fluids.json');
const entities = read_json_file('entities.json');
const recipes = read_json_file('recipes.json');

// https://lua-api.factorio.com/0.16.51/LuaItemPrototype.html
const items_rust = 'use super::model::*;\n' + Object.values(items).map(item => {
    const { name, fuel_value } = item;
    return `
static ${constant_case(name)}: Item = Item {
    name: "${name}",
    fuel_value: ${printFloat(fuel_value)},
};
`;
}).join('');

// https://lua-api.factorio.com/0.16.51/LuaFluidPrototype.html
const fluids_rust = 'use super::model::*;\n' + Object.values(fluids).map(fluid => {
    const { name, fuel_value } = fluid;
    return `
static ${constant_case(name)}: Fluid = Fluid {
    name: "${name}",
    fuel_value: ${printFloat(fuel_value)},
};
`;
}).join('');

function entity_energy_to_rust(entity) {
    const { energy_usage, burner_prototype: burner, electric_energy_source_prototype: electric } = entity;

    if (isObject(burner) && !isObject(electric)) {
        return `EntityEnergy::Fueled {
        usage: ${isNumber(entity.energy_usage) ? printFloat(entity.energy_usage) : printFloat(entity.max_energy_usage)},
        effectivity: ${printFloat(burner.effectivity)},
    }`;
    }
    if (!isObject(burner) && isObject(electric)) {
        return `EntityEnergy::Electric {
        usage: ${isNumber(entity.energy_usage) ? printFloat(entity.energy_usage) : printFloat(entity.max_energy_usage)},
        drain: ${printFloat(electric.drain)},
    }`;
    }
    if (!isObject(burner) && !isObject(electric)) {
        return `EntityEnergy::None`;
    }
    console.error(entity);
    throw new Error('Entity with both burner and electric energy sources.');
}

// https://lua-api.factorio.com/0.16.51/LuaEntityPrototype.html
const entities_rust = 'use super::model::*;\n' + Object.values(entities).map(entity => {
    const { name, crafting_speed } = entity;
    return `
static ${constant_case(name)}: Entity = Entity {
    name: "${name}",
    crafting_speed: ${printOptionFloat(crafting_speed)},
    energy: ${entity_energy_to_rust(entity)},
};
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
            amount: ${printFloat(amount)},
        },`;
        }
        if (type == "fluid") {
            const { minimum_temperature, maximum_temperature } = ingredient;
            if (isNumber(minimum_temperature) && isNumber(maximum_temperature)) {
                return `
        Ingredient::FluidTemp {
            name: "${name}",
            amount: ${printFloat(amount)},
            minimum_temperature: ${minimum_temperature < 0 ? 0 : minimum_temperature},
            maximum_temperature: ${maximum_temperature},
        },`;
            } else {
                return `
        Ingredient::Fluid {
            name: "${name}",
            amount: ${printFloat(amount)},
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
            amount: ${printFloat(amount)},
        },`;
        }
        const { amount_min, amount_max, probability } = product;
        if (isNumber(amount_min) && isNumber(amount_max) && isNumber(probability)) {
            return `
        Product::ItemChance {
            name: "${name}",
            amount_min: ${printFloat(amount_min)},
            amount_max: ${printFloat(amount_max)},
            probability: ${printFloat(probability)},
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
            amount: ${printFloat(amount)},
            temperature: ${temperature},
        },`;
            } else {
                return `
        Product::Fluid {
            name: "${name}",
            amount: ${printFloat(amount)},
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
            amount_min: ${printFloat(amount_min)},
            amount_max: ${printFloat(amount_max)},
            probability: ${printFloat(probability)},
            temperature: ${temperature},
        },`;
            } else {
                return `
        Product::FluidChance {
            name: "${name}",
            amount_min: ${printFloat(amount_min)},
            amount_max: ${printFloat(amount_max)},
            probability: ${printFloat(probability)},
        },`;
            }
        }
    }
    console.error(product);
    throw new Error("Invalid product.");
}

const recipes_rust = 'use super::model::*;\n' + Object.values(recipes).map(recipe => {
    return `
static ${constant_case(recipe.name)}: Recipe = Recipe {
    name: "${recipe.name}",
    ingredients: &[${recipe.ingredients.map(ingredient_to_rust).join('')}
    ],
    products: &[${recipe.products.map(product_to_rust).join('')}
    ],
    time: ${printFloat(recipe.energy)}
};
`;
}).join('');

fs.writeFileSync('../src/items.rs', items_rust);
fs.writeFileSync('../src/fluids.rs', fluids_rust);
fs.writeFileSync('../src/entities.rs', entities_rust);
fs.writeFileSync('../src/recipes.rs', recipes_rust);
