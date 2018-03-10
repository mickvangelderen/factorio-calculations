use super::*;

pub static water_pumping: Process = Process {
    name: "water_pumping",
    ingredients: &[
        Ingredient {
            material: Material::Water,
            quantity: 1.0,
        },
    ],
    time: 1.0,
};

pub static dirt_water_electrolysis: Process = Process {
    name: "dirt_water_electrolysis",
    ingredients: &[
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
    time: 2.0,
};

pub static mineralized_water_crystallization: Process = Process {
    name: "mineralized_water_crystallization",
    ingredients: &[
        Ingredient {
            material: Material::MineralizedWater,
            quantity: -200.0,
        },
        Ingredient {
            material: Material::SapphiriteOre,
            quantity: 1.1,
        },
        Ingredient {
            material: Material::SteratiteOre,
            quantity: 0.7,
        },
    ],
    time: 2.0,
};

pub static sapphirite_ore_crushing: Process = Process {
    name: "sapphirite_ore_crushing",
    ingredients: &[
        Ingredient {
            material: Material::SapphiriteOre,
            quantity: -2.0,
        },
        Ingredient {
            material: Material::CrushedStone,
            quantity: 1.0,
        },
        Ingredient {
            material: Material::CrushedSapphirite,
            quantity: 2.0,
        },
    ],
    time: 1.0,
};

pub static steratite_ore_crushing: Process = Process {
    name: "steratite_ore_crushing",
    ingredients: &[
        Ingredient {
            material: Material::SteratiteOre,
            quantity: -2.0,
        },
        Ingredient {
            material: Material::CrushedStone,
            quantity: 1.0,
        },
        Ingredient {
            material: Material::CrushedSteratite,
            quantity: 2.0,
        },
    ],
    time: 1.0,
};

pub static crushed_sapphirite_to_iron_plate: Process = Process {
    name: "crushed_sapphirite_to_iron_plate",
    ingredients: &[
        Ingredient {
            material: Material::CrushedSapphirite,
            quantity: -3.0,
        },
        Ingredient {
            material: Material::IronPlate,
            quantity: 2.0,
        },
    ],
    time: 7.0,
};

pub static crushed_steratite_to_copper_plate: Process = Process {
    name: "crushed_steratite_to_copper_plate",
    ingredients: &[
        Ingredient {
            material: Material::CrushedSteratite,
            quantity: -3.0,
        },
        Ingredient {
            material: Material::CopperPlate,
            quantity: 2.0,
        },
    ],
    time: 7.0,
};

pub static crushed_sapphirite_ore_sorting: Process = Process {
    name: "crushed_sapphirite_ore_sorting",
    ingredients: &[
        Ingredient {
            material: Material::CrushedSapphirite,
            quantity: -4.0,
        },
        Ingredient {
            material: Material::Slag,
            quantity: 1.0,
        },
        Ingredient {
            material: Material::IronOre,
            quantity: 2.0,
        },
        Ingredient {
            material: Material::CopperOre,
            quantity: 1.0,
        },
    ],
    time: 1.0,
};

pub static crushed_steratite_ore_sorting: Process = Process {
    name: "crushed_steratite_ore_sorting",
    ingredients: &[
        Ingredient {
            material: Material::CrushedSteratite,
            quantity: -4.0,
        },
        Ingredient {
            material: Material::Slag,
            quantity: 1.0,
        },
        Ingredient {
            material: Material::CopperOre,
            quantity: 2.0,
        },
        Ingredient {
            material: Material::IronOre,
            quantity: 1.0,
        },
    ],
    time: 1.0,
};

pub static iron_ore_to_iron_plate: Process = Process {
    name: "iron_ore_to_iron_plate",
    ingredients: &[
        Ingredient {
            material: Material::IronOre,
            quantity: -4.0,
        },
        Ingredient {
            material: Material::IronPlate,
            quantity: 3.0,
        },
    ],
    time: 10.5,
};

pub static copper_ore_to_copper_plate: Process = Process {
    name: "copper_ore_to_copper_plate",
    ingredients: &[
        Ingredient {
            material: Material::CopperOre,
            quantity: -4.0,
        },
        Ingredient {
            material: Material::CopperPlate,
            quantity: 3.0,
        },
    ],
    time: 10.5,
};

pub static stone_crushing: Process = Process {
    name: "stone_crushing",
    ingredients: &[
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

pub static water_mineralization: Process = Process {
    name: "water_mineralization",
    ingredients: &[
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

pub static green_algae_growing: Process = Process {
    name: "green_algae_growing",
    ingredients: &[
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

pub static green_algae_to_fiber: Process = Process {
    name: "green_algae_to_fiber",
    ingredients: &[
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

pub static fiber_to_wood_pellet: Process = Process {
    name: "fiber_to_wood_pellet",
    ingredients: &[
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

pub static wood_pellet_to_carbon_dioxide: Process = Process {
    name: "wood_pellet_to_carbon_dioxide",
    ingredients: &[
        Ingredient {
            material: Material::WoodPellet,
            quantity: -1.0,
        },
        Ingredient {
            material: Material::CarbonDioxide,
            quantity: 70.0,
        },
    ],
    time: 2.0
};

pub static wood_pellet_to_wood_brick: Process = Process {
    name: "wood_pellet_to_wood_brick",
    ingredients: &[
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

pub static wood_brick_to_charcoal: Process = Process {
    name: "wood_brick_to_charcoal",
    ingredients: &[
        Ingredient {
            material: Material::WoodBrick,
            quantity: -1.0,
        },
        Ingredient {
            material: Material::Charcoal,
            quantity: 6.0,
        },
    ],
    time: 3.5,
};

pub static charcoal_to_carbon_dioxide: Process = Process {
    name: "charcoal_to_carbon_dioxide",
    ingredients: &[
        Ingredient {
            material: Material::Charcoal,
            quantity: -1.0,
        },
        Ingredient {
            material: Material::CarbonDioxide,
            quantity: 25.0,
        },
    ],
    time: 1.0,
};

pub static charcoal_to_carbon: Process = Process {
    name: "charcoal_to_carbon",
    ingredients: &[
        Ingredient {
            material: Material::Charcoal,
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

pub static water_purification: Process = Process {
    name: "water_purification",
    ingredients: &[
        Ingredient {
            material: Material::Water,
            quantity: -150.0,
        },
        Ingredient {
            material: Material::SalineWater,
            quantity: 20.0,
        },
        Ingredient {
            material: Material::PurifiedWater,
            quantity: 100.0,
        },
    ],
    time: 1.0,
};

// pub static clean_coal_to_coke_and_sulfuric_waste_water: Process = Process {
//     name: "clean_coal_to_coke_and_sulfuric_waste_water",
//     ingredients: &[
//         Ingredient {
//             material: Material::CrushedCoal,
//             quantity: -2.0,
//         },
//         Ingredient {
//             material: Material::PurifiedWater,
//             quantity: -50.0,
//         },
//         Ingredient {
//             material: Material::Coke,
//             quantity: 2.0,
//         },
//         Ingredient {
//             material: Material::SulfuricWasteWater,
//             quantity: 50.0,
//         },
//     ],
//     time: 4.0,
// };

pub static sulfuric_waste_water_purification: Process = Process {
    name: "sulfuric_waste_water_purification",
    ingredients: &[
        Ingredient {
            material: Material::SulfuricWasteWater,
            quantity: -100.0,
        },
        Ingredient {
            material: Material::Sulfur,
            quantity: 1.0,
        },
        Ingredient {
            material: Material::MineralizedWater,
            quantity: 20.0,
        },
        Ingredient {
            material: Material::PurifiedWater,
            quantity: 70.0,
        },
    ],
    time: 1.0,
};

pub static void_saline_water: Process = Process {
    name: "void_saline_water",
    ingredients: &[
        Ingredient {
            material: Material::SalineWater,
            quantity: -400.0,
        },
    ],
    time: 5.0,
};

pub static boiler_mk1_power: Process = Process {
    name: "boiler_mk1_power",
    // 50% efficiency.
    ingredients: &[
        Ingredient {
            material: Material::Joule,
            quantity: 0.5 * 3_600_000.0,
        },
    ],
    time: 1.0,
};

pub static boiler_mk2_power: Process = Process {
    name: "boiler_mk2_power",
    // 60% efficiency.
    ingredients: &[
        Ingredient {
            material: Material::Joule,
            quantity: 0.6 * 3_600_000.0,
        },
    ],
    time: 1.0,
};

pub static burn_oxygen: Process = Process {
    name: "burn_oxygen",
    ingredients: &[
        Ingredient {
            material: Material::Oxygen,
            quantity: -100.0,
        },
    ],
    time: 1.0,
};

pub static burn_hydrogen: Process = Process {
    name: "burn_hydrogen",
    ingredients: &[
        Ingredient {
            material: Material::Hydrogen,
            quantity: -100.0,
        },
    ],
    time: 1.0,
};

pub static pump_viscous_mud_water: Process = Process {
    name: "pump_viscous_mud_water",
    ingredients: &[
        Ingredient {
            material: Material::ViscousMudWater,
            quantity: 1.0,
        },
    ],
    time: 1.0,
};

pub static wash_viscous_mud_water: Process = Process {
    name: "wash_viscous_mud_water",
    ingredients: &[
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

pub static wash_heavy_mud_water: Process = Process {
    name: "wash_heavy_mud_water",
    ingredients: &[
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

pub static heavy_mud_water_to_nodule: Process = Process {
    name: "heavy_mud_water_to_nodule",
    ingredients: &[
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
