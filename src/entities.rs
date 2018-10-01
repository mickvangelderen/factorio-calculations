use super::model::*;

static WOODEN_CHEST: Entity = Entity {
    name: "wooden-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static IRON_CHEST: Entity = Entity {
    name: "iron-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STEEL_CHEST: Entity = Entity {
    name: "steel-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BRASS_CHEST: Entity = Entity {
    name: "brass-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TITANIUM_CHEST: Entity = Entity {
    name: "titanium-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STORAGE_TANK_2: Entity = Entity {
    name: "storage-tank-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STORAGE_TANK_3: Entity = Entity {
    name: "storage-tank-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STORAGE_TANK_4: Entity = Entity {
    name: "storage-tank-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOADER: Entity = Entity {
    name: "loader",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FAST_LOADER: Entity = Entity {
    name: "fast-loader",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPRESS_LOADER: Entity = Entity {
    name: "express-loader",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BURNER_GENERATOR_POWER: Entity = Entity {
    name: "burner-generator-power",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BURNER_INSERTER: Entity = Entity {
    name: "burner-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 3140.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static INSERTER: Entity = Entity {
    name: "inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 220.000000000000000000,
        drain: 6.66666666666669982533,
    },
};

static LONG_HANDED_INSERTER: Entity = Entity {
    name: "long-handed-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 328.500000000000000000,
        drain: 6.66666666666669982533,
    },
};

static FILTER_INSERTER: Entity = Entity {
    name: "filter-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 880.000000000000000000,
        drain: 8.33333333333330017467,
    },
};

static FAST_INSERTER: Entity = Entity {
    name: "fast-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 770.000000000000000000,
        drain: 8.33333333333330017467,
    },
};

static STACK_INSERTER: Entity = Entity {
    name: "stack-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 2200.00000000000000000,
        drain: 16.6666666666669982533,
    },
};

static STACK_FILTER_INSERTER: Entity = Entity {
    name: "stack-filter-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 2200.00000000000000000,
        drain: 16.6666666666669982533,
    },
};

static EXPRESS_INSERTER: Entity = Entity {
    name: "express-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 3500.00000000000000000,
        drain: 11.6666666666670000296,
    },
};

static EXPRESS_FILTER_INSERTER: Entity = Entity {
    name: "express-filter-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 3500.00000000000000000,
        drain: 11.6666666666670000296,
    },
};

static EXPRESS_STACK_INSERTER: Entity = Entity {
    name: "express-stack-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 8750.00000000000000000,
        drain: 25.0000000000000000000,
    },
};

static EXPRESS_STACK_FILTER_INSERTER: Entity = Entity {
    name: "express-stack-filter-inserter",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 8750.00000000000000000,
        drain: 25.0000000000000000000,
    },
};

static SMALL_ELECTRIC_POLE: Entity = Entity {
    name: "small-electric-pole",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_ELECTRIC_POLE: Entity = Entity {
    name: "medium-electric-pole",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_ELECTRIC_POLE_2: Entity = Entity {
    name: "medium-electric-pole-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_ELECTRIC_POLE_3: Entity = Entity {
    name: "medium-electric-pole-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_ELECTRIC_POLE_4: Entity = Entity {
    name: "medium-electric-pole-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_ELECTRIC_POLE: Entity = Entity {
    name: "big-electric-pole",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_ELECTRIC_POLE_2: Entity = Entity {
    name: "big-electric-pole-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_ELECTRIC_POLE_3: Entity = Entity {
    name: "big-electric-pole-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_ELECTRIC_POLE_4: Entity = Entity {
    name: "big-electric-pole-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SUBSTATION: Entity = Entity {
    name: "substation",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SUBSTATION_2: Entity = Entity {
    name: "substation-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SUBSTATION_3: Entity = Entity {
    name: "substation-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SUBSTATION_4: Entity = Entity {
    name: "substation-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_VALVE: Entity = Entity {
    name: "bob-valve",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PUMP: Entity = Entity {
    name: "pump",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_PUMP_2: Entity = Entity {
    name: "bob-pump-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 666.666666666670039376,
        drain: 0.00000000000000000000,
    },
};

static BOB_PUMP_3: Entity = Entity {
    name: "bob-pump-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 833.333333333329960624,
        drain: 0.00000000000000000000,
    },
};

static BOB_PUMP_4: Entity = Entity {
    name: "bob-pump-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static CURVED_RAIL: Entity = Entity {
    name: "curved-rail",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STRAIGHT_RAIL: Entity = Entity {
    name: "straight-rail",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TRAIN_STOP: Entity = Entity {
    name: "train-stop",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static RAIL_SIGNAL: Entity = Entity {
    name: "rail-signal",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static RAIL_CHAIN_SIGNAL: Entity = Entity {
    name: "rail-chain-signal",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ARTILLERY_WAGON: Entity = Entity {
    name: "artillery-wagon",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARTILLERY_WAGON_2: Entity = Entity {
    name: "bob-artillery-wagon-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARTILLERY_WAGON_3: Entity = Entity {
    name: "bob-artillery-wagon-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CAR: Entity = Entity {
    name: "car",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 2500.00000000000000000,
        effectivity: 0.599999999999999977796,
    },
};

static TANK: Entity = Entity {
    name: "tank",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 10000.0000000000000000,
        effectivity: 0.750000000000000000000,
    },
};

static BOB_TANK_2: Entity = Entity {
    name: "bob-tank-2",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 13333.3333333329992456,
        effectivity: 0.849999999999999977796,
    },
};

static BOB_TANK_3: Entity = Entity {
    name: "bob-tank-3",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 16666.6666666669989354,
        effectivity: 1.00000000000000000000,
    },
};

static LOCOMOTIVE: Entity = Entity {
    name: "locomotive",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 10000.0000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static BOB_LOCOMOTIVE_2: Entity = Entity {
    name: "bob-locomotive-2",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 15000.0000000000000000,
        effectivity: 1.19999999999999995559,
    },
};

static BOB_LOCOMOTIVE_3: Entity = Entity {
    name: "bob-locomotive-3",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 20000.0000000000000000,
        effectivity: 1.39999999999999991118,
    },
};

static BOB_ARMOURED_LOCOMOTIVE: Entity = Entity {
    name: "bob-armoured-locomotive",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 12500.0000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static BOB_ARMOURED_LOCOMOTIVE_2: Entity = Entity {
    name: "bob-armoured-locomotive-2",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 16666.6666666669989354,
        effectivity: 1.19999999999999995559,
    },
};

static CARGO_WAGON: Entity = Entity {
    name: "cargo-wagon",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_CARGO_WAGON_2: Entity = Entity {
    name: "bob-cargo-wagon-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_CARGO_WAGON_3: Entity = Entity {
    name: "bob-cargo-wagon-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARMOURED_CARGO_WAGON: Entity = Entity {
    name: "bob-armoured-cargo-wagon",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARMOURED_CARGO_WAGON_2: Entity = Entity {
    name: "bob-armoured-cargo-wagon-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FLUID_WAGON: Entity = Entity {
    name: "fluid-wagon",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_FLUID_WAGON_2: Entity = Entity {
    name: "bob-fluid-wagon-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_FLUID_WAGON_3: Entity = Entity {
    name: "bob-fluid-wagon-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARMOURED_FLUID_WAGON: Entity = Entity {
    name: "bob-armoured-fluid-wagon",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARMOURED_FLUID_WAGON_2: Entity = Entity {
    name: "bob-armoured-fluid-wagon-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_ACTIVE_PROVIDER: Entity = Entity {
    name: "logistic-chest-active-provider",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_BUFFER: Entity = Entity {
    name: "logistic-chest-buffer",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_PASSIVE_PROVIDER: Entity = Entity {
    name: "logistic-chest-passive-provider",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_REQUESTER: Entity = Entity {
    name: "logistic-chest-requester",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_STORAGE: Entity = Entity {
    name: "logistic-chest-storage",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_ACTIVE_PROVIDER_2: Entity = Entity {
    name: "logistic-chest-active-provider-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_BUFFER_2: Entity = Entity {
    name: "logistic-chest-buffer-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_PASSIVE_PROVIDER_2: Entity = Entity {
    name: "logistic-chest-passive-provider-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_REQUESTER_2: Entity = Entity {
    name: "logistic-chest-requester-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_STORAGE_2: Entity = Entity {
    name: "logistic-chest-storage-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_ACTIVE_PROVIDER_3: Entity = Entity {
    name: "logistic-chest-active-provider-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_BUFFER_3: Entity = Entity {
    name: "logistic-chest-buffer-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_PASSIVE_PROVIDER_3: Entity = Entity {
    name: "logistic-chest-passive-provider-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_REQUESTER_3: Entity = Entity {
    name: "logistic-chest-requester-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_CHEST_STORAGE_3: Entity = Entity {
    name: "logistic-chest-storage-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_LAMP: Entity = Entity {
    name: "small-lamp",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 83.3333333333330017467,
        drain: 0.00000000000000000000,
    },
};

static ARITHMETIC_COMBINATOR: Entity = Entity {
    name: "arithmetic-combinator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 16.6666666666669982533,
        drain: 0.00000000000000000000,
    },
};

static DECIDER_COMBINATOR: Entity = Entity {
    name: "decider-combinator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 16.6666666666669982533,
        drain: 0.00000000000000000000,
    },
};

static CONSTANT_COMBINATOR: Entity = Entity {
    name: "constant-combinator",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SPACEX_COMBINATOR: Entity = Entity {
    name: "spacex-combinator",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POWER_SWITCH: Entity = Entity {
    name: "power-switch",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PROGRAMMABLE_SPEAKER: Entity = Entity {
    name: "programmable-speaker",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 33.3333333333330017467,
        drain: 0.00000000000000000000,
    },
};

static TRANSPORT_BELT: Entity = Entity {
    name: "transport-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static UNDERGROUND_BELT: Entity = Entity {
    name: "underground-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SPLITTER: Entity = Entity {
    name: "splitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FAST_TRANSPORT_BELT: Entity = Entity {
    name: "fast-transport-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FAST_UNDERGROUND_BELT: Entity = Entity {
    name: "fast-underground-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FAST_SPLITTER: Entity = Entity {
    name: "fast-splitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPRESS_TRANSPORT_BELT: Entity = Entity {
    name: "express-transport-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPRESS_UNDERGROUND_BELT: Entity = Entity {
    name: "express-underground-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPRESS_SPLITTER: Entity = Entity {
    name: "express-splitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TURBO_TRANSPORT_BELT: Entity = Entity {
    name: "turbo-transport-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TURBO_UNDERGROUND_BELT: Entity = Entity {
    name: "turbo-underground-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TURBO_SPLITTER: Entity = Entity {
    name: "turbo-splitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ULTIMATE_TRANSPORT_BELT: Entity = Entity {
    name: "ultimate-transport-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ULTIMATE_UNDERGROUND_BELT: Entity = Entity {
    name: "ultimate-underground-belt",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ULTIMATE_SPLITTER: Entity = Entity {
    name: "ultimate-splitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static COPPER_PIPE: Entity = Entity {
    name: "copper-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PIPE: Entity = Entity {
    name: "pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STONE_PIPE: Entity = Entity {
    name: "stone-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BRONZE_PIPE: Entity = Entity {
    name: "bronze-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STEEL_PIPE: Entity = Entity {
    name: "steel-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PLASTIC_PIPE: Entity = Entity {
    name: "plastic-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BRASS_PIPE: Entity = Entity {
    name: "brass-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TITANIUM_PIPE: Entity = Entity {
    name: "titanium-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CERAMIC_PIPE: Entity = Entity {
    name: "ceramic-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TUNGSTEN_PIPE: Entity = Entity {
    name: "tungsten-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static COPPER_PIPE_TO_GROUND: Entity = Entity {
    name: "copper-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PIPE_TO_GROUND: Entity = Entity {
    name: "pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STONE_PIPE_TO_GROUND: Entity = Entity {
    name: "stone-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BRONZE_PIPE_TO_GROUND: Entity = Entity {
    name: "bronze-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STEEL_PIPE_TO_GROUND: Entity = Entity {
    name: "steel-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PLASTIC_PIPE_TO_GROUND: Entity = Entity {
    name: "plastic-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BRASS_PIPE_TO_GROUND: Entity = Entity {
    name: "brass-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TITANIUM_PIPE_TO_GROUND: Entity = Entity {
    name: "titanium-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CERAMIC_PIPE_TO_GROUND: Entity = Entity {
    name: "ceramic-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TUNGSTEN_PIPE_TO_GROUND: Entity = Entity {
    name: "tungsten-pipe-to-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LOGISTIC_ROBOT: Entity = Entity {
    name: "logistic-robot",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LOGISTIC_ROBOT_2: Entity = Entity {
    name: "bob-logistic-robot-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LOGISTIC_ROBOT_3: Entity = Entity {
    name: "bob-logistic-robot-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LOGISTIC_ROBOT_4: Entity = Entity {
    name: "bob-logistic-robot-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LOGISTIC_ROBOT_5: Entity = Entity {
    name: "bob-logistic-robot-5",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CONSTRUCTION_ROBOT: Entity = Entity {
    name: "construction-robot",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_CONSTRUCTION_ROBOT_2: Entity = Entity {
    name: "bob-construction-robot-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_CONSTRUCTION_ROBOT_3: Entity = Entity {
    name: "bob-construction-robot-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_CONSTRUCTION_ROBOT_4: Entity = Entity {
    name: "bob-construction-robot-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_CONSTRUCTION_ROBOT_5: Entity = Entity {
    name: "bob-construction-robot-5",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROBOPORT: Entity = Entity {
    name: "roboport",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 833.333333333329960624,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOPORT_2: Entity = Entity {
    name: "bob-roboport-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOPORT_3: Entity = Entity {
    name: "bob-roboport-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOPORT_4: Entity = Entity {
    name: "bob-roboport-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 0.00000000000000000000,
    },
};

static BOB_LOGISTIC_ZONE_INTERFACE: Entity = Entity {
    name: "bob-logistic-zone-interface",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 166.666666666670010954,
        drain: 0.00000000000000000000,
    },
};

static BOB_LOGISTIC_ZONE_EXPANDER: Entity = Entity {
    name: "bob-logistic-zone-expander",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 416.666666666669982533,
        drain: 0.00000000000000000000,
    },
};

static BOB_LOGISTIC_ZONE_EXPANDER_2: Entity = Entity {
    name: "bob-logistic-zone-expander-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 833.333333333329960624,
        drain: 0.00000000000000000000,
    },
};

static BOB_LOGISTIC_ZONE_EXPANDER_3: Entity = Entity {
    name: "bob-logistic-zone-expander-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1250.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_LOGISTIC_ZONE_EXPANDER_4: Entity = Entity {
    name: "bob-logistic-zone-expander-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOCHEST: Entity = Entity {
    name: "bob-robochest",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 83.3333333333330017467,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOCHEST_2: Entity = Entity {
    name: "bob-robochest-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 83.3333333333330017467,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOCHEST_3: Entity = Entity {
    name: "bob-robochest-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 83.3333333333330017467,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBOCHEST_4: Entity = Entity {
    name: "bob-robochest-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 83.3333333333330017467,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT: Entity = Entity {
    name: "bob-robo-charge-port",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 83333.3333333329937886,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_2: Entity = Entity {
    name: "bob-robo-charge-port-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 208333.333333329996094,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_3: Entity = Entity {
    name: "bob-robo-charge-port-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 333333.333333329996094,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_4: Entity = Entity {
    name: "bob-robo-charge-port-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 500000.000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_LARGE: Entity = Entity {
    name: "bob-robo-charge-port-large",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 187500.000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_LARGE_2: Entity = Entity {
    name: "bob-robo-charge-port-large-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 468750.000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_LARGE_3: Entity = Entity {
    name: "bob-robo-charge-port-large-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 750000.000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BOB_ROBO_CHARGE_PORT_LARGE_4: Entity = Entity {
    name: "bob-robo-charge-port-large-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1125000.00000000000000,
        drain: 0.00000000000000000000,
    },
};

static OIL_STEAM_BOILER: Entity = Entity {
    name: "OilSteamBoiler",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static BURNER_GENERATOR: Entity = Entity {
    name: "burner-generator",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 7516.66666666669971164,
        effectivity: 0.250000000000000000000,
    },
};

static WIND_TURBINE_2: Entity = Entity {
    name: "wind-turbine-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static PETROLEUM_GENERATOR: Entity = Entity {
    name: "petroleum-generator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static ELECTRIC_ENERGY_INTERFACE: Entity = Entity {
    name: "electric-energy-interface",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static CENTRIFUGE: Entity = Entity {
    name: "centrifuge",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5833.33333333330028836,
        drain: 194.444444444439994868,
    },
};

static NUCLEAR_REACTOR: Entity = Entity {
    name: "nuclear-reactor",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 666666.666666670003906,
        effectivity: 1.00000000000000000000,
    },
};

static HEAT_PIPE: Entity = Entity {
    name: "heat-pipe",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOILER: Entity = Entity {
    name: "boiler",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 30000.0000000000000000,
        effectivity: 0.500000000000000000000,
    },
};

static BOILER_2: Entity = Entity {
    name: "boiler-2",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 60000.0000000000000000,
        effectivity: 0.599999999999999977796,
    },
};

static BOILER_3: Entity = Entity {
    name: "boiler-3",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 90000.0000000000000000,
        effectivity: 0.699999999999999955591,
    },
};

static BOILER_4: Entity = Entity {
    name: "boiler-4",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 120000.000000000000000,
        effectivity: 0.800000000000000044409,
    },
};

static BOILER_5: Entity = Entity {
    name: "boiler-5",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 150000.000000000000000,
        effectivity: 0.900000000000000022204,
    },
};

static HEAT_EXCHANGER: Entity = Entity {
    name: "heat-exchanger",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static HEAT_EXCHANGER_2: Entity = Entity {
    name: "heat-exchanger-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static HEAT_EXCHANGER_3: Entity = Entity {
    name: "heat-exchanger-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STEAM_ENGINE: Entity = Entity {
    name: "steam-engine",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_ENGINE_2: Entity = Entity {
    name: "steam-engine-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_ENGINE_3: Entity = Entity {
    name: "steam-engine-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_ENGINE_4: Entity = Entity {
    name: "steam-engine-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_ENGINE_5: Entity = Entity {
    name: "steam-engine-5",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_TURBINE: Entity = Entity {
    name: "steam-turbine",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_TURBINE_2: Entity = Entity {
    name: "steam-turbine-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_TURBINE_3: Entity = Entity {
    name: "steam-turbine-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static FLUID_GENERATOR: Entity = Entity {
    name: "fluid-generator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static FLUID_GENERATOR_2: Entity = Entity {
    name: "fluid-generator-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static FLUID_GENERATOR_3: Entity = Entity {
    name: "fluid-generator-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static HYDRAZINE_GENERATOR: Entity = Entity {
    name: "hydrazine-generator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_SMALL: Entity = Entity {
    name: "solar-panel-small",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL: Entity = Entity {
    name: "solar-panel",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_LARGE: Entity = Entity {
    name: "solar-panel-large",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_SMALL_2: Entity = Entity {
    name: "solar-panel-small-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_2: Entity = Entity {
    name: "solar-panel-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_LARGE_2: Entity = Entity {
    name: "solar-panel-large-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_SMALL_3: Entity = Entity {
    name: "solar-panel-small-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_3: Entity = Entity {
    name: "solar-panel-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SOLAR_PANEL_LARGE_3: Entity = Entity {
    name: "solar-panel-large-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static ACCUMULATOR: Entity = Entity {
    name: "accumulator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static FAST_ACCUMULATOR: Entity = Entity {
    name: "fast-accumulator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 4000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static LARGE_ACCUMULATOR: Entity = Entity {
    name: "large-accumulator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 10000.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SLOW_ACCUMULATOR: Entity = Entity {
    name: "slow-accumulator",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 4000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static FAST_ACCUMULATOR_2: Entity = Entity {
    name: "fast-accumulator-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 6000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static LARGE_ACCUMULATOR_2: Entity = Entity {
    name: "large-accumulator-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 15000.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SLOW_ACCUMULATOR_2: Entity = Entity {
    name: "slow-accumulator-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 6000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static FAST_ACCUMULATOR_3: Entity = Entity {
    name: "fast-accumulator-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 9000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static LARGE_ACCUMULATOR_3: Entity = Entity {
    name: "large-accumulator-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 22500.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SLOW_ACCUMULATOR_3: Entity = Entity {
    name: "slow-accumulator-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 9000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static BURNER_MINING_DRILL: Entity = Entity {
    name: "burner-mining-drill",
    crafting_speed: None,
    energy: EntityEnergy::Fueled {
        usage: 5000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static ELECTRIC_MINING_DRILL: Entity = Entity {
    name: "electric-mining-drill",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1500.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static OFFSHORE_PUMP: Entity = Entity {
    name: "offshore-pump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PUMPJACK: Entity = Entity {
    name: "pumpjack",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1500.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static AIR_PUMP: Entity = Entity {
    name: "air-pump",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 833.333333333329960624,
        drain: 27.7777777777779988355,
    },
};

static AIR_PUMP_2: Entity = Entity {
    name: "air-pump-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1500.00000000000000000,
        drain: 50.0000000000000000000,
    },
};

static AIR_PUMP_3: Entity = Entity {
    name: "air-pump-3",
    crafting_speed: Some(3.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2333.33333333329983361,
        drain: 77.7777777777779988355,
    },
};

static AIR_PUMP_4: Entity = Entity {
    name: "air-pump-4",
    crafting_speed: Some(5.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2916.66666666670016639,
        drain: 97.2222222222220011645,
    },
};

static WATER_PUMP: Entity = Entity {
    name: "water-pump",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 833.333333333329960624,
        drain: 27.7777777777779988355,
    },
};

static WATER_PUMP_2: Entity = Entity {
    name: "water-pump-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1500.00000000000000000,
        drain: 50.0000000000000000000,
    },
};

static WATER_PUMP_3: Entity = Entity {
    name: "water-pump-3",
    crafting_speed: Some(3.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2333.33333333329983361,
        drain: 77.7777777777779988355,
    },
};

static WATER_PUMP_4: Entity = Entity {
    name: "water-pump-4",
    crafting_speed: Some(5.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2916.66666666670016639,
        drain: 97.2222222222220011645,
    },
};

static VOID_PUMP: Entity = Entity {
    name: "void-pump",
    crafting_speed: Some(5.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static STONE_FURNACE: Entity = Entity {
    name: "stone-furnace",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static STEEL_FURNACE: Entity = Entity {
    name: "steel-furnace",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static ELECTRIC_FURNACE: Entity = Entity {
    name: "electric-furnace",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3000.00000000000000000,
        drain: 100.000000000000000000,
    },
};

static ELECTRIC_FURNACE_2: Entity = Entity {
    name: "electric-furnace-2",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ELECTRIC_FURNACE_3: Entity = Entity {
    name: "electric-furnace-3",
    crafting_speed: Some(4.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static CHEMICAL_BOILER: Entity = Entity {
    name: "chemical-boiler",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static CHEMICAL_STEEL_FURNACE: Entity = Entity {
    name: "chemical-steel-furnace",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static MIXING_FURNACE: Entity = Entity {
    name: "mixing-furnace",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static MIXING_STEEL_FURNACE: Entity = Entity {
    name: "mixing-steel-furnace",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static CHEMICAL_FURNACE: Entity = Entity {
    name: "chemical-furnace",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3000.00000000000000000,
        drain: 100.000000000000000000,
    },
};

static ELECTRIC_MIXING_FURNACE: Entity = Entity {
    name: "electric-mixing-furnace",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3000.00000000000000000,
        drain: 100.000000000000000000,
    },
};

static ELECTRIC_CHEMICAL_MIXING_FURNACE: Entity = Entity {
    name: "electric-chemical-mixing-furnace",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ELECTRIC_CHEMICAL_MIXING_FURNACE_2: Entity = Entity {
    name: "electric-chemical-mixing-furnace-2",
    crafting_speed: Some(4.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static BOB_DISTILLERY: Entity = Entity {
    name: "bob-distillery",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3000.00000000000000000,
        drain: 100.000000000000000000,
    },
};

static ASSEMBLING_MACHINE_1: Entity = Entity {
    name: "assembling-machine-1",
    crafting_speed: Some(0.500000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static ASSEMBLING_MACHINE_2: Entity = Entity {
    name: "assembling-machine-2",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2250.00000000000000000,
        drain: 75.0000000000000000000,
    },
};

static ASSEMBLING_MACHINE_3: Entity = Entity {
    name: "assembling-machine-3",
    crafting_speed: Some(1.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3500.00000000000000000,
        drain: 116.666666666669996744,
    },
};

static ASSEMBLING_MACHINE_4: Entity = Entity {
    name: "assembling-machine-4",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ASSEMBLING_MACHINE_5: Entity = Entity {
    name: "assembling-machine-5",
    crafting_speed: Some(2.75000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6500.00000000000000000,
        drain: 216.666666666670010954,
    },
};

static ASSEMBLING_MACHINE_6: Entity = Entity {
    name: "assembling-machine-6",
    crafting_speed: Some(3.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 8000.00000000000000000,
        drain: 266.666666666669982533,
    },
};

static ELECTRONICS_MACHINE_1: Entity = Entity {
    name: "electronics-machine-1",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static ELECTRONICS_MACHINE_2: Entity = Entity {
    name: "electronics-machine-2",
    crafting_speed: Some(2.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3562.50000000000000000,
        drain: 118.750000000000000000,
    },
};

static ELECTRONICS_MACHINE_3: Entity = Entity {
    name: "electronics-machine-3",
    crafting_speed: Some(4.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6000.00000000000000000,
        drain: 200.000000000000000000,
    },
};

static CHEMICAL_PLANT_2: Entity = Entity {
    name: "chemical-plant-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static CHEMICAL_PLANT_3: Entity = Entity {
    name: "chemical-plant-3",
    crafting_speed: Some(2.75000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6500.00000000000000000,
        drain: 216.666666666670010954,
    },
};

static CHEMICAL_PLANT_4: Entity = Entity {
    name: "chemical-plant-4",
    crafting_speed: Some(3.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 8000.00000000000000000,
        drain: 266.666666666669982533,
    },
};

static ELECTROLYSER: Entity = Entity {
    name: "electrolyser",
    crafting_speed: Some(0.800000000000000044409),
    energy: EntityEnergy::Electric {
        usage: 7500.00000000000000000,
        drain: 250.000000000000000000,
    },
};

static ELECTROLYSER_2: Entity = Entity {
    name: "electrolyser-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 12500.0000000000000000,
        drain: 416.666666666669982533,
    },
};

static ELECTROLYSER_3: Entity = Entity {
    name: "electrolyser-3",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 18750.0000000000000000,
        drain: 625.000000000000000000,
    },
};

static ELECTROLYSER_4: Entity = Entity {
    name: "electrolyser-4",
    crafting_speed: Some(3.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 23333.3333333330010646,
        drain: 777.777777777779988355,
    },
};

static BEACON: Entity = Entity {
    name: "beacon",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 8000.00000000000000000,
        drain: 8000.00000000000000000,
    },
};

static BEACON_2: Entity = Entity {
    name: "beacon-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 16000.0000000000000000,
        drain: 16000.0000000000000000,
    },
};

static BEACON_3: Entity = Entity {
    name: "beacon-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 32000.0000000000000000,
        drain: 32000.0000000000000000,
    },
};

static LAB: Entity = Entity {
    name: "lab",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SCT_LAB_T2: Entity = Entity {
    name: "sct-lab-t2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static SCT_LAB_T3: Entity = Entity {
    name: "sct-lab-t3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 8333.33333333329937886,
        drain: 0.00000000000000000000,
    },
};

static SCT_LAB_T4: Entity = Entity {
    name: "sct-lab-t4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 33333.3333333330010646,
        drain: 0.00000000000000000000,
    },
};

static LAB_2: Entity = Entity {
    name: "lab-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 50000.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static LAB_ALIEN: Entity = Entity {
    name: "lab-alien",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1250.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static LAB_MODULE: Entity = Entity {
    name: "lab-module",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 1250.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static DISTRACTOR_MINE: Entity = Entity {
    name: "distractor-mine",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LAND_MINE: Entity = Entity {
    name: "land-mine",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POISON_MINE: Entity = Entity {
    name: "poison-mine",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SLOWDOWN_MINE: Entity = Entity {
    name: "slowdown-mine",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ROBOT_TANK: Entity = Entity {
    name: "bob-robot-tank",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STONE_WALL: Entity = Entity {
    name: "stone-wall",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GATE: Entity = Entity {
    name: "gate",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static REINFORCED_WALL: Entity = Entity {
    name: "reinforced-wall",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static REINFORCED_GATE: Entity = Entity {
    name: "reinforced-gate",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GUN_TURRET: Entity = Entity {
    name: "gun-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GUN_TURRET_2: Entity = Entity {
    name: "bob-gun-turret-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GUN_TURRET_3: Entity = Entity {
    name: "bob-gun-turret-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GUN_TURRET_4: Entity = Entity {
    name: "bob-gun-turret-4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GUN_TURRET_5: Entity = Entity {
    name: "bob-gun-turret-5",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LASER_TURRET: Entity = Entity {
    name: "laser-turret",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 40000.0000000000000000,
        drain: 400.000000000000000000,
    },
};

static BOB_LASER_TURRET_2: Entity = Entity {
    name: "bob-laser-turret-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 50000.0000000000000000,
        drain: 500.000000000000000000,
    },
};

static BOB_LASER_TURRET_3: Entity = Entity {
    name: "bob-laser-turret-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 60000.0000000000000000,
        drain: 600.000000000000000000,
    },
};

static BOB_LASER_TURRET_4: Entity = Entity {
    name: "bob-laser-turret-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 70000.0000000000000000,
        drain: 700.000000000000000000,
    },
};

static BOB_LASER_TURRET_5: Entity = Entity {
    name: "bob-laser-turret-5",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 80000.0000000000000000,
        drain: 800.000000000000000000,
    },
};

static FLAMETHROWER_TURRET: Entity = Entity {
    name: "flamethrower-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_SNIPER_TURRET_1: Entity = Entity {
    name: "bob-sniper-turret-1",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_SNIPER_TURRET_2: Entity = Entity {
    name: "bob-sniper-turret-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_SNIPER_TURRET_3: Entity = Entity {
    name: "bob-sniper-turret-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ARTILLERY_TURRET: Entity = Entity {
    name: "artillery-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARTILLERY_TURRET_2: Entity = Entity {
    name: "bob-artillery-turret-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ARTILLERY_TURRET_3: Entity = Entity {
    name: "bob-artillery-turret-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static RADAR: Entity = Entity {
    name: "radar",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static RADAR_2: Entity = Entity {
    name: "radar-2",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 7500.00000000000000000,
        drain: 0.00000000000000000000,
    },
};

static RADAR_3: Entity = Entity {
    name: "radar-3",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 10000.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static RADAR_4: Entity = Entity {
    name: "radar-4",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 12500.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static RADAR_5: Entity = Entity {
    name: "radar-5",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 15000.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static ROCKET_SILO: Entity = Entity {
    name: "rocket-silo",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static PLAYER_PORT: Entity = Entity {
    name: "player-port",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ORE_SORTING_FACILITY: Entity = Entity {
    name: "ore-sorting-facility",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ORE_SORTING_FACILITY_2: Entity = Entity {
    name: "ore-sorting-facility-2",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static ORE_SORTING_FACILITY_3: Entity = Entity {
    name: "ore-sorting-facility-3",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ORE_SORTING_FACILITY_4: Entity = Entity {
    name: "ore-sorting-facility-4",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static BURNER_ORE_CRUSHER: Entity = Entity {
    name: "burner-ore-crusher",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 1666.66666666669993901,
        effectivity: 1.00000000000000000000,
    },
};

static ORE_CRUSHER: Entity = Entity {
    name: "ore-crusher",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static ORE_CRUSHER_2: Entity = Entity {
    name: "ore-crusher-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2083.33333333329983361,
        drain: 69.4444444444440023290,
    },
};

static ORE_CRUSHER_3: Entity = Entity {
    name: "ore-crusher-3",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ORE_POWDERIZER: Entity = Entity {
    name: "ore-powderizer",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static ORE_FLOATATION_CELL: Entity = Entity {
    name: "ore-floatation-cell",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static ORE_POWDERIZER_2: Entity = Entity {
    name: "ore-powderizer-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2083.33333333329983361,
        drain: 69.4444444444440023290,
    },
};

static ORE_FLOATATION_CELL_2: Entity = Entity {
    name: "ore-floatation-cell-2",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ORE_POWDERIZER_3: Entity = Entity {
    name: "ore-powderizer-3",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ORE_FLOATATION_CELL_3: Entity = Entity {
    name: "ore-floatation-cell-3",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ELECTRO_WHINNING_CELL: Entity = Entity {
    name: "electro-whinning-cell",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static ELECTRO_WHINNING_CELL_2: Entity = Entity {
    name: "electro-whinning-cell-2",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ELECTRO_WHINNING_CELL_3: Entity = Entity {
    name: "electro-whinning-cell-3",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ORE_LEACHING_PLANT: Entity = Entity {
    name: "ore-leaching-plant",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static ORE_LEACHING_PLANT_2: Entity = Entity {
    name: "ore-leaching-plant-2",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ORE_LEACHING_PLANT_3: Entity = Entity {
    name: "ore-leaching-plant-3",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ORE_REFINERY: Entity = Entity {
    name: "ore-refinery",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ORE_REFINERY_2: Entity = Entity {
    name: "ore-refinery-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5833.33333333330028836,
        drain: 194.444444444439994868,
    },
};

static THERMAL_EXTRACTOR: Entity = Entity {
    name: "thermal-extractor",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1500.00000000000000000,
        drain: 50.0000000000000000000,
    },
};

static FILTRATION_UNIT: Entity = Entity {
    name: "filtration-unit",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static FILTRATION_UNIT_2: Entity = Entity {
    name: "filtration-unit-2",
    crafting_speed: Some(2.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static CRYSTALLIZER: Entity = Entity {
    name: "crystallizer",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static CRYSTALLIZER_2: Entity = Entity {
    name: "crystallizer-2",
    crafting_speed: Some(2.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ANGELS_WAREHOUSE: Entity = Entity {
    name: "angels-warehouse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_WAREHOUSE_PASSIVE_PROVIDER: Entity = Entity {
    name: "angels-warehouse-passive-provider",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_WAREHOUSE_ACTIVE_PROVIDER: Entity = Entity {
    name: "angels-warehouse-active-provider",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_WAREHOUSE_STORAGE: Entity = Entity {
    name: "angels-warehouse-storage",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_WAREHOUSE_REQUESTER: Entity = Entity {
    name: "angels-warehouse-requester",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static HYDRO_PLANT: Entity = Entity {
    name: "hydro-plant",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static HYDRO_PLANT_2: Entity = Entity {
    name: "hydro-plant-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static CLARIFIER: Entity = Entity {
    name: "clarifier",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static SALINATION_PLANT: Entity = Entity {
    name: "salination-plant",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static SALINATION_PLANT_2: Entity = Entity {
    name: "salination-plant-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static COOLING_TOWER: Entity = Entity {
    name: "cooling-tower",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static SEAFLOOR_PUMP: Entity = Entity {
    name: "seafloor-pump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static WASHING_PLANT: Entity = Entity {
    name: "washing-plant",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static WASHING_PLANT_2: Entity = Entity {
    name: "washing-plant-2",
    crafting_speed: Some(2.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static STORAGE_TANK: Entity = Entity {
    name: "storage-tank",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static VALVE_CHECK: Entity = Entity {
    name: "valve-check",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static VALVE_RETURN: Entity = Entity {
    name: "valve-return",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static VALVE_OVERFLOW: Entity = Entity {
    name: "valve-overflow",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static VALVE_CONVERTER: Entity = Entity {
    name: "valve-converter",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static VALVE_UNDERFLOW: Entity = Entity {
    name: "valve-underflow",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BARRELING_PUMP: Entity = Entity {
    name: "barreling-pump",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1000.00000000000000000,
        drain: 33.3333333333330017467,
    },
};

static ANGELS_STORAGE_TANK_1: Entity = Entity {
    name: "angels-storage-tank-1",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_STORAGE_TANK_2: Entity = Entity {
    name: "angels-storage-tank-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_STORAGE_TANK_3: Entity = Entity {
    name: "angels-storage-tank-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ELECTROLYSER: Entity = Entity {
    name: "angels-electrolyser",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ANGELS_ELECTROLYSER_2: Entity = Entity {
    name: "angels-electrolyser-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5833.33333333330028836,
        drain: 194.444444444439994868,
    },
};

static ANGELS_ELECTROLYSER_3: Entity = Entity {
    name: "angels-electrolyser-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6666.66666666669971164,
        drain: 222.222222222220011645,
    },
};

static ANGELS_ELECTROLYSER_4: Entity = Entity {
    name: "angels-electrolyser-4",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 7500.00000000000000000,
        drain: 250.000000000000000000,
    },
};

static LIQUIFIER: Entity = Entity {
    name: "liquifier",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2083.33333333329983361,
        drain: 69.4444444444440023290,
    },
};

static LIQUIFIER_2: Entity = Entity {
    name: "liquifier-2",
    crafting_speed: Some(2.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static LIQUIFIER_3: Entity = Entity {
    name: "liquifier-3",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static LIQUIFIER_4: Entity = Entity {
    name: "liquifier-4",
    crafting_speed: Some(3.75000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ANGELS_AIR_FILTER: Entity = Entity {
    name: "angels-air-filter",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ANGELS_AIR_FILTER_2: Entity = Entity {
    name: "angels-air-filter-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static SEPARATOR: Entity = Entity {
    name: "separator",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static SEPARATOR_2: Entity = Entity {
    name: "separator-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static SEPARATOR_3: Entity = Entity {
    name: "separator-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static SEPARATOR_4: Entity = Entity {
    name: "separator-4",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5416.66666666669971164,
        drain: 180.555555555560005132,
    },
};

static GAS_REFINERY_SMALL: Entity = Entity {
    name: "gas-refinery-small",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static GAS_REFINERY_SMALL_2: Entity = Entity {
    name: "gas-refinery-small-2",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5833.33333333330028836,
        drain: 194.444444444439994868,
    },
};

static GAS_REFINERY_SMALL_3: Entity = Entity {
    name: "gas-refinery-small-3",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6666.66666666669971164,
        drain: 222.222222222220011645,
    },
};

static GAS_REFINERY_SMALL_4: Entity = Entity {
    name: "gas-refinery-small-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 7083.33333333330028836,
        drain: 236.111111111110005822,
    },
};

static ANGELS_FLARE_STACK: Entity = Entity {
    name: "angels-flare-stack",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static ANGELS_ELECTRIC_BOILER: Entity = Entity {
    name: "angels-electric-boiler",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 20000.0000000000000000,
        drain: 0.00000000000000000000,
    },
};

static STEAM_CRACKER: Entity = Entity {
    name: "steam-cracker",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static STEAM_CRACKER_2: Entity = Entity {
    name: "steam-cracker-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3750.00000000000000000,
        drain: 125.000000000000000000,
    },
};

static STEAM_CRACKER_3: Entity = Entity {
    name: "steam-cracker-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static STEAM_CRACKER_4: Entity = Entity {
    name: "steam-cracker-4",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4583.33333333330028836,
        drain: 152.777777777779988355,
    },
};

static GAS_REFINERY: Entity = Entity {
    name: "gas-refinery",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6666.66666666669971164,
        drain: 222.222222222220011645,
    },
};

static GAS_REFINERY_2: Entity = Entity {
    name: "gas-refinery-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 7500.00000000000000000,
        drain: 250.000000000000000000,
    },
};

static GAS_REFINERY_3: Entity = Entity {
    name: "gas-refinery-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 8333.33333333329937886,
        drain: 277.777777777779988355,
    },
};

static GAS_REFINERY_4: Entity = Entity {
    name: "gas-refinery-4",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 10000.0000000000000000,
        drain: 333.333333333330017467,
    },
};

static ADVANCED_CHEMICAL_PLANT: Entity = Entity {
    name: "advanced-chemical-plant",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ADVANCED_CHEMICAL_PLANT_2: Entity = Entity {
    name: "advanced-chemical-plant-2",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6666.66666666669971164,
        drain: 222.222222222220011645,
    },
};

static CHEMICAL_PLANT: Entity = Entity {
    name: "chemical-plant",
    crafting_speed: Some(1.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3500.00000000000000000,
        drain: 116.666666666669996744,
    },
};

static ANGELS_CHEMICAL_PLANT: Entity = Entity {
    name: "angels-chemical-plant",
    crafting_speed: Some(1.75000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ANGELS_CHEMICAL_PLANT_2: Entity = Entity {
    name: "angels-chemical-plant-2",
    crafting_speed: Some(2.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static ANGELS_CHEMICAL_PLANT_3: Entity = Entity {
    name: "angels-chemical-plant-3",
    crafting_speed: Some(2.75000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5833.33333333330028836,
        drain: 194.444444444439994868,
    },
};

static ANGELS_CHEMICAL_PLANT_4: Entity = Entity {
    name: "angels-chemical-plant-4",
    crafting_speed: Some(3.25000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 6666.66666666669971164,
        drain: 222.222222222220011645,
    },
};

static OIL_REFINERY: Entity = Entity {
    name: "oil-refinery",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 7000.00000000000000000,
        drain: 233.333333333329989046,
    },
};

static OIL_REFINERY_2: Entity = Entity {
    name: "oil-refinery-2",
    crafting_speed: Some(1.75000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 12000.0000000000000000,
        drain: 400.000000000000000000,
    },
};

static OIL_REFINERY_3: Entity = Entity {
    name: "oil-refinery-3",
    crafting_speed: Some(2.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 16666.6666666669989354,
        drain: 555.555555555559976710,
    },
};

static OIL_REFINERY_4: Entity = Entity {
    name: "oil-refinery-4",
    crafting_speed: Some(3.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 22500.0000000000000000,
        drain: 750.000000000000000000,
    },
};

static ALGAE_FARM: Entity = Entity {
    name: "algae-farm",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2000.00000000000000000,
        drain: 66.6666666666669982533,
    },
};

static ALGAE_FARM_2: Entity = Entity {
    name: "algae-farm-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_ARBORETUM_1: Entity = Entity {
    name: "bio-arboretum-1",
    crafting_speed: Some(0.500000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static BIO_GENERATOR_DESERT_1: Entity = Entity {
    name: "bio-generator-desert-1",
    crafting_speed: Some(0.500000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static BIO_GENERATOR_SWAMP_1: Entity = Entity {
    name: "bio-generator-swamp-1",
    crafting_speed: Some(0.500000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static BIO_GENERATOR_TEMPERATE_1: Entity = Entity {
    name: "bio-generator-temperate-1",
    crafting_speed: Some(0.500000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static CROP_FARM: Entity = Entity {
    name: "crop-farm",
    crafting_speed: Some(0.500000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static TEMPERATE_FARM: Entity = Entity {
    name: "temperate-farm",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2083.33333333329983361,
        drain: 69.4444444444440023290,
    },
};

static DESERT_FARM: Entity = Entity {
    name: "desert-farm",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static SWAMP_FARM: Entity = Entity {
    name: "swamp-farm",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static BIO_PRESS: Entity = Entity {
    name: "bio-press",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_PROCESSOR: Entity = Entity {
    name: "bio-processor",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static COMPOSTER: Entity = Entity {
    name: "composter",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 500.000000000000000000,
        drain: 16.6666666666669982533,
    },
};

static NUTRIENT_EXTRACTOR: Entity = Entity {
    name: "nutrient-extractor",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static SEED_EXTRACTOR: Entity = Entity {
    name: "seed-extractor",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_BUTCHERY: Entity = Entity {
    name: "bio-butchery",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_HATCHERY: Entity = Entity {
    name: "bio-hatchery",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_REFUGIUM_BITER: Entity = Entity {
    name: "bio-refugium-biter",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_REFUGIUM_FISH: Entity = Entity {
    name: "bio-refugium-fish",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_REFUGIUM_HOGGER: Entity = Entity {
    name: "bio-refugium-hogger",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static BIO_REFUGIUM_PUFFER: Entity = Entity {
    name: "bio-refugium-puffer",
    crafting_speed: Some(0.750000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ORE_PROCESSING_MACHINE: Entity = Entity {
    name: "ore-processing-machine",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ORE_PROCESSING_MACHINE_2: Entity = Entity {
    name: "ore-processing-machine-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static ORE_PROCESSING_MACHINE_3: Entity = Entity {
    name: "ore-processing-machine-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ORE_PROCESSING_MACHINE_4: Entity = Entity {
    name: "ore-processing-machine-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static PELLET_PRESS: Entity = Entity {
    name: "pellet-press",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static PELLET_PRESS_2: Entity = Entity {
    name: "pellet-press-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static PELLET_PRESS_3: Entity = Entity {
    name: "pellet-press-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static PELLET_PRESS_4: Entity = Entity {
    name: "pellet-press-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5833.33333333330028836,
        drain: 194.444444444439994868,
    },
};

static POWDER_MIXER: Entity = Entity {
    name: "powder-mixer",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 1666.66666666669993901,
        drain: 55.5555555555559976710,
    },
};

static POWDER_MIXER_2: Entity = Entity {
    name: "powder-mixer-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2083.33333333329983361,
        drain: 69.4444444444440023290,
    },
};

static POWDER_MIXER_3: Entity = Entity {
    name: "powder-mixer-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static POWDER_MIXER_4: Entity = Entity {
    name: "powder-mixer-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2916.66666666670016639,
        drain: 97.2222222222220011645,
    },
};

static BLAST_FURNACE: Entity = Entity {
    name: "blast-furnace",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 2500.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static BLAST_FURNACE_2: Entity = Entity {
    name: "blast-furnace-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 3333.33333333329983361,
        effectivity: 1.00000000000000000000,
    },
};

static BLAST_FURNACE_3: Entity = Entity {
    name: "blast-furnace-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 4166.66666666669971164,
        effectivity: 1.00000000000000000000,
    },
};

static BLAST_FURNACE_4: Entity = Entity {
    name: "blast-furnace-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Fueled {
        usage: 5000.00000000000000000,
        effectivity: 1.00000000000000000000,
    },
};

static ANGELS_CHEMICAL_FURNACE: Entity = Entity {
    name: "angels-chemical-furnace",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static ANGELS_CHEMICAL_FURNACE_2: Entity = Entity {
    name: "angels-chemical-furnace-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static ANGELS_CHEMICAL_FURNACE_3: Entity = Entity {
    name: "angels-chemical-furnace-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static ANGELS_CHEMICAL_FURNACE_4: Entity = Entity {
    name: "angels-chemical-furnace-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 5000.00000000000000000,
        drain: 166.666666666670010954,
    },
};

static INDUCTION_FURNACE: Entity = Entity {
    name: "induction-furnace",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static INDUCTION_FURNACE_2: Entity = Entity {
    name: "induction-furnace-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static INDUCTION_FURNACE_3: Entity = Entity {
    name: "induction-furnace-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static INDUCTION_FURNACE_4: Entity = Entity {
    name: "induction-furnace-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static CASTING_MACHINE: Entity = Entity {
    name: "casting-machine",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static CASTING_MACHINE_2: Entity = Entity {
    name: "casting-machine-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static CASTING_MACHINE_3: Entity = Entity {
    name: "casting-machine-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static CASTING_MACHINE_4: Entity = Entity {
    name: "casting-machine-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static STRAND_CASTING_MACHINE: Entity = Entity {
    name: "strand-casting-machine",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static STRAND_CASTING_MACHINE_2: Entity = Entity {
    name: "strand-casting-machine-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static STRAND_CASTING_MACHINE_3: Entity = Entity {
    name: "strand-casting-machine-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static STRAND_CASTING_MACHINE_4: Entity = Entity {
    name: "strand-casting-machine-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static SINTERING_OVEN: Entity = Entity {
    name: "sintering-oven",
    crafting_speed: Some(1.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 2500.00000000000000000,
        drain: 83.3333333333330017467,
    },
};

static SINTERING_OVEN_2: Entity = Entity {
    name: "sintering-oven-2",
    crafting_speed: Some(1.50000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 3333.33333333329983361,
        drain: 111.111111111110005822,
    },
};

static SINTERING_OVEN_3: Entity = Entity {
    name: "sintering-oven-3",
    crafting_speed: Some(2.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static SINTERING_OVEN_4: Entity = Entity {
    name: "sintering-oven-4",
    crafting_speed: Some(3.00000000000000000000),
    energy: EntityEnergy::Electric {
        usage: 4166.66666666669971164,
        drain: 138.888888888889994178,
    },
};

static INFINITY_CHEST: Entity = Entity {
    name: "infinity-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SIMPLE_ENTITY_WITH_FORCE: Entity = Entity {
    name: "simple-entity-with-force",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SIMPLE_ENTITY_WITH_OWNER: Entity = Entity {
    name: "simple-entity-with-owner",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CLIFF: Entity = Entity {
    name: "cliff",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_CLIFF: Entity = Entity {
    name: "small-cliff",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PLAYER: Entity = Entity {
    name: "player",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FISH: Entity = Entity {
    name: "fish",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ALIEN_FISH_1: Entity = Entity {
    name: "alien-fish-1",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ALIEN_FISH_2: Entity = Entity {
    name: "alien-fish-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ALIEN_FISH_3: Entity = Entity {
    name: "alien-fish-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_01: Entity = Entity {
    name: "tree-01",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_02: Entity = Entity {
    name: "tree-02",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_03: Entity = Entity {
    name: "tree-03",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_04: Entity = Entity {
    name: "tree-04",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_05: Entity = Entity {
    name: "tree-05",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_09: Entity = Entity {
    name: "tree-09",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_02_RED: Entity = Entity {
    name: "tree-02-red",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_07: Entity = Entity {
    name: "tree-07",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_06: Entity = Entity {
    name: "tree-06",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_06_BROWN: Entity = Entity {
    name: "tree-06-brown",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_09_BROWN: Entity = Entity {
    name: "tree-09-brown",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_09_RED: Entity = Entity {
    name: "tree-09-red",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_08: Entity = Entity {
    name: "tree-08",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_08_BROWN: Entity = Entity {
    name: "tree-08-brown",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_08_RED: Entity = Entity {
    name: "tree-08-red",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DEAD_DRY_HAIRY_TREE: Entity = Entity {
    name: "dead-dry-hairy-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DEAD_GREY_TRUNK: Entity = Entity {
    name: "dead-grey-trunk",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DEAD_TREE_DESERT: Entity = Entity {
    name: "dead-tree-desert",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DESERT_GARDEN: Entity = Entity {
    name: "desert-garden",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DRY_HAIRY_TREE: Entity = Entity {
    name: "dry-hairy-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PUFFER_NEST: Entity = Entity {
    name: "puffer-nest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SWAMP_GARDEN: Entity = Entity {
    name: "swamp-garden",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TEMPERATE_GARDEN: Entity = Entity {
    name: "temperate-garden",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DRY_TREE: Entity = Entity {
    name: "dry-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TEMPERATE_TREE: Entity = Entity {
    name: "temperate-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DESERT_TREE: Entity = Entity {
    name: "desert-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SWAMP_TREE: Entity = Entity {
    name: "swamp-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_CRYSTAL_ROCK: Entity = Entity {
    name: "angels-crystal-rock",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROCK_HUGE: Entity = Entity {
    name: "rock-huge",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROCK_BIG: Entity = Entity {
    name: "rock-big",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_BITER_CORPSE: Entity = Entity {
    name: "small-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_BITER_CORPSE: Entity = Entity {
    name: "medium-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BEHEMOTH_BITER_CORPSE: Entity = Entity {
    name: "behemoth-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_BITER_CORPSE: Entity = Entity {
    name: "big-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_PIERCING_BITER_CORPSE: Entity = Entity {
    name: "bob-big-piercing-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_ACID_BITER_CORPSE: Entity = Entity {
    name: "bob-huge-acid-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_EXPLOSIVE_BITER_CORPSE: Entity = Entity {
    name: "bob-huge-explosive-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BEHEMOTH_BITER_CORPSE: Entity = Entity {
    name: "bob-behemoth-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_FIRE_BITER_CORPSE: Entity = Entity {
    name: "bob-giant-fire-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_POISON_BITER_CORPSE: Entity = Entity {
    name: "bob-giant-poison-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_TITAN_BITER_CORPSE: Entity = Entity {
    name: "bob-titan-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LEVIATHAN_BITER_CORPSE: Entity = Entity {
    name: "bob-leviathan-biter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BITER_SPAWNER_CORPSE: Entity = Entity {
    name: "biter-spawner-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_SPITTER_CORPSE: Entity = Entity {
    name: "small-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_SPITTER_CORPSE: Entity = Entity {
    name: "medium-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_SPITTER_CORPSE: Entity = Entity {
    name: "big-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_ELECTRIC_SPITTER_CORPSE: Entity = Entity {
    name: "bob-big-electric-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BEHEMOTH_SPITTER_CORPSE: Entity = Entity {
    name: "behemoth-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_ACID_SPITTER_CORPSE: Entity = Entity {
    name: "bob-huge-acid-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_EXPLOSIVE_SPITTER_CORPSE: Entity = Entity {
    name: "bob-huge-explosive-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BEHEMOTH_SPITTER_CORPSE: Entity = Entity {
    name: "bob-behemoth-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_FIRE_SPITTER_CORPSE: Entity = Entity {
    name: "bob-giant-fire-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_POISON_SPITTER_CORPSE: Entity = Entity {
    name: "bob-giant-poison-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_TITAN_SPITTER_CORPSE: Entity = Entity {
    name: "bob-titan-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LEVIATHAN_SPITTER_CORPSE: Entity = Entity {
    name: "bob-leviathan-spitter-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_SPITTER_SPAWNER_CORPSE: Entity = Entity {
    name: "bob-spitter-spawner-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SPITTER_SPAWNER_CORPSE: Entity = Entity {
    name: "spitter-spawner-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_WORM_CORPSE: Entity = Entity {
    name: "small-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_WORM_CORPSE: Entity = Entity {
    name: "medium-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_WORM_CORPSE: Entity = Entity {
    name: "big-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_EXPLOSIVE_WORM_CORPSE: Entity = Entity {
    name: "bob-big-explosive-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_FIRE_WORM_CORPSE: Entity = Entity {
    name: "bob-big-fire-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_POISON_WORM_CORPSE: Entity = Entity {
    name: "bob-big-poison-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_PIERCING_WORM_CORPSE: Entity = Entity {
    name: "bob-big-piercing-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_ELECTRIC_WORM_CORPSE: Entity = Entity {
    name: "bob-big-electric-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_WORM_CORPSE: Entity = Entity {
    name: "bob-giant-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BEHEMOTH_WORM_CORPSE: Entity = Entity {
    name: "bob-behemoth-worm-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BITER_SPAWNER_CORPSE: Entity = Entity {
    name: "bob-biter-spawner-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_REMNANTS: Entity = Entity {
    name: "small-remnants",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_REMNANTS: Entity = Entity {
    name: "medium-remnants",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_REMNANTS: Entity = Entity {
    name: "big-remnants",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STRAIGHT_RAIL_REMNANTS: Entity = Entity {
    name: "straight-rail-remnants",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CURVED_RAIL_REMNANTS: Entity = Entity {
    name: "curved-rail-remnants",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_SCORCHMARK: Entity = Entity {
    name: "small-scorchmark",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_01_STUMP: Entity = Entity {
    name: "tree-01-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_02_STUMP: Entity = Entity {
    name: "tree-02-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_03_STUMP: Entity = Entity {
    name: "tree-03-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_04_STUMP: Entity = Entity {
    name: "tree-04-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_05_STUMP: Entity = Entity {
    name: "tree-05-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_06_STUMP: Entity = Entity {
    name: "tree-06-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_07_STUMP: Entity = Entity {
    name: "tree-07-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_08_STUMP: Entity = Entity {
    name: "tree-08-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TREE_09_STUMP: Entity = Entity {
    name: "tree-09-stump",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static WALL_REMNANTS: Entity = Entity {
    name: "wall-remnants",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SAND_ROCK_BIG: Entity = Entity {
    name: "sand-rock-big",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_SHIP_WRECK_1: Entity = Entity {
    name: "big-ship-wreck-1",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_SHIP_WRECK_2: Entity = Entity {
    name: "big-ship-wreck-2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_SHIP_WRECK_3: Entity = Entity {
    name: "big-ship-wreck-3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_SHIP_WRECK: Entity = Entity {
    name: "medium-ship-wreck",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_SHIP_WRECK: Entity = Entity {
    name: "small-ship-wreck",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_BITER: Entity = Entity {
    name: "small-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_BITER: Entity = Entity {
    name: "medium-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_BITER: Entity = Entity {
    name: "big-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_PIERCING_BITER: Entity = Entity {
    name: "bob-big-piercing-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BEHEMOTH_BITER: Entity = Entity {
    name: "behemoth-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_ACID_BITER: Entity = Entity {
    name: "bob-huge-acid-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_EXPLOSIVE_BITER: Entity = Entity {
    name: "bob-huge-explosive-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_SPITTER: Entity = Entity {
    name: "small-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_WORM_TURRET: Entity = Entity {
    name: "small-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_FIRE_BITER: Entity = Entity {
    name: "bob-giant-fire-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_POISON_BITER: Entity = Entity {
    name: "bob-giant-poison-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_SPITTER: Entity = Entity {
    name: "medium-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_WORM_TURRET: Entity = Entity {
    name: "medium-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BEHEMOTH_SPITTER: Entity = Entity {
    name: "behemoth-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_SPITTER: Entity = Entity {
    name: "big-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_WORM_TURRET: Entity = Entity {
    name: "big-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_ELECTRIC_SPITTER: Entity = Entity {
    name: "bob-big-electric-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_TITAN_BITER: Entity = Entity {
    name: "bob-titan-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BITER_SPAWNER: Entity = Entity {
    name: "biter-spawner",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BEHEMOTH_BITER: Entity = Entity {
    name: "bob-behemoth-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_ELECTRIC_WORM_TURRET: Entity = Entity {
    name: "bob-big-electric-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_EXPLOSIVE_WORM_TURRET: Entity = Entity {
    name: "bob-big-explosive-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_FIRE_WORM_TURRET: Entity = Entity {
    name: "bob-big-fire-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_PIERCING_WORM_TURRET: Entity = Entity {
    name: "bob-big-piercing-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BIG_POISON_WORM_TURRET: Entity = Entity {
    name: "bob-big-poison-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_ACID_SPITTER: Entity = Entity {
    name: "bob-huge-acid-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_HUGE_EXPLOSIVE_SPITTER: Entity = Entity {
    name: "bob-huge-explosive-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LEVIATHAN_BITER: Entity = Entity {
    name: "bob-leviathan-biter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_FIRE_SPITTER: Entity = Entity {
    name: "bob-giant-fire-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_POISON_SPITTER: Entity = Entity {
    name: "bob-giant-poison-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SPITTER_SPAWNER: Entity = Entity {
    name: "spitter-spawner",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BITER_SPAWNER: Entity = Entity {
    name: "bob-biter-spawner",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GIANT_WORM_TURRET: Entity = Entity {
    name: "bob-giant-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_TITAN_SPITTER: Entity = Entity {
    name: "bob-titan-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BEHEMOTH_SPITTER: Entity = Entity {
    name: "bob-behemoth-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BEHEMOTH_WORM_TURRET: Entity = Entity {
    name: "bob-behemoth-worm-turret",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LEVIATHAN_SPITTER: Entity = Entity {
    name: "bob-leviathan-spitter",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_SPITTER_SPAWNER: Entity = Entity {
    name: "bob-spitter-spawner",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MARKET: Entity = Entity {
    name: "market",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DEFENDER: Entity = Entity {
    name: "defender",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DISTRACTOR: Entity = Entity {
    name: "distractor",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DESTROYER: Entity = Entity {
    name: "destroyer",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_ROBOT: Entity = Entity {
    name: "bob-laser-robot",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ACID_PROJECTILE_PURPLE: Entity = Entity {
    name: "acid-projectile-purple",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ACID_SPLASH_PURPLE: Entity = Entity {
    name: "acid-splash-purple",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE1_PARTICLE: Entity = Entity {
    name: "angels-ore1-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE2_PARTICLE: Entity = Entity {
    name: "angels-ore2-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE3_PARTICLE: Entity = Entity {
    name: "angels-ore3-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE4_PARTICLE: Entity = Entity {
    name: "angels-ore4-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE5_PARTICLE: Entity = Entity {
    name: "angels-ore5-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE6_PARTICLE: Entity = Entity {
    name: "angels-ore6-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static AREA_ACID_PROJECTILE_PURPLE: Entity = Entity {
    name: "area-acid-projectile-purple",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ARTILLERY_CANNON_MUZZLE_FLASH: Entity = Entity {
    name: "artillery-cannon-muzzle-flash",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ARTILLERY_FLARE: Entity = Entity {
    name: "artillery-flare",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ARTILLERY_PROJECTILE: Entity = Entity {
    name: "artillery-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ATOMIC_BOMB_WAVE: Entity = Entity {
    name: "atomic-bomb-wave",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ATOMIC_ROCKET: Entity = Entity {
    name: "atomic-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BEHEMOTH_SPIT_PROJECTILE: Entity = Entity {
    name: "behemoth-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BETTER_SHOTGUN_PROJECTILE: Entity = Entity {
    name: "better-shotgun-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_ARTILLERY_EXPLOSION: Entity = Entity {
    name: "big-artillery-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_EXPLOSION: Entity = Entity {
    name: "big-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BIG_SHIP_WRECK_GRASS: Entity = Entity {
    name: "big-ship-wreck-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLASTING_PROJECTILE: Entity = Entity {
    name: "blasting-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLOOD_EXPLOSION_BIG: Entity = Entity {
    name: "blood-explosion-big",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLOOD_EXPLOSION_HUGE: Entity = Entity {
    name: "blood-explosion-huge",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLOOD_EXPLOSION_SMALL: Entity = Entity {
    name: "blood-explosion-small",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLOOD_FOUNTAIN: Entity = Entity {
    name: "blood-fountain",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLOOD_FOUNTAIN_BIG: Entity = Entity {
    name: "blood-fountain-big",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLOOD_PARTICLE: Entity = Entity {
    name: "blood-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLUE_LASER: Entity = Entity {
    name: "blue-laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ACID_ROCKET: Entity = Entity {
    name: "bob-acid-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_BLUE_LASER: Entity = Entity {
    name: "bob-blue-laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ELECTRIC_ROCKET: Entity = Entity {
    name: "bob-electric-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_EXPLOSIVE_ROCKET: Entity = Entity {
    name: "bob-explosive-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_FLAME_ROCKET: Entity = Entity {
    name: "bob-flame-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_GREEN_LASER: Entity = Entity {
    name: "bob-green-laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_AMETHYST: Entity = Entity {
    name: "bob-laser-amethyst",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_AMETHYST: Entity = Entity {
    name: "bob-laser-bubble-amethyst",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_DIAMOND: Entity = Entity {
    name: "bob-laser-bubble-diamond",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_EMERALD: Entity = Entity {
    name: "bob-laser-bubble-emerald",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_GLASS: Entity = Entity {
    name: "bob-laser-bubble-glass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_RUBY: Entity = Entity {
    name: "bob-laser-bubble-ruby",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_SAPPHIRE: Entity = Entity {
    name: "bob-laser-bubble-sapphire",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_BUBBLE_TOPAZ: Entity = Entity {
    name: "bob-laser-bubble-topaz",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_DIAMOND: Entity = Entity {
    name: "bob-laser-diamond",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_EMERALD: Entity = Entity {
    name: "bob-laser-emerald",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_GLASS: Entity = Entity {
    name: "bob-laser-glass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_ROBOT_CAPSULE: Entity = Entity {
    name: "bob-laser-robot-capsule",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_RUBY: Entity = Entity {
    name: "bob-laser-ruby",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_SAPPHIRE: Entity = Entity {
    name: "bob-laser-sapphire",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_LASER_TOPAZ: Entity = Entity {
    name: "bob-laser-topaz",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_PIERCING_ROCKET: Entity = Entity {
    name: "bob-piercing-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_POISON_ROCKET: Entity = Entity {
    name: "bob-poison-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_PURPLE_LASER: Entity = Entity {
    name: "bob-purple-laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_ROCKET: Entity = Entity {
    name: "bob-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_WHITE_LASER: Entity = Entity {
    name: "bob-white-laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BOB_YELLOW_LASER: Entity = Entity {
    name: "bob-yellow-laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BRANCH_PARTICLE: Entity = Entity {
    name: "branch-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_ASTERISK: Entity = Entity {
    name: "brown-asterisk",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_CANE_CLUSTER: Entity = Entity {
    name: "brown-cane-cluster",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_CANE_SINGLE: Entity = Entity {
    name: "brown-cane-single",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_CARPET_GRASS: Entity = Entity {
    name: "brown-carpet-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_CORAL_MINI: Entity = Entity {
    name: "brown-coral-mini",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_FLUFF: Entity = Entity {
    name: "brown-fluff",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_FLUFF_DRY: Entity = Entity {
    name: "brown-fluff-dry",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BROWN_HAIRY_GRASS: Entity = Entity {
    name: "brown-hairy-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CANNON_PROJECTILE: Entity = Entity {
    name: "cannon-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CANNON_PROJECTILE_PELLET: Entity = Entity {
    name: "cannon-projectile-pellet",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CHARACTER_CORPSE: Entity = Entity {
    name: "character-corpse",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CLIFF_EXPLOSIVES: Entity = Entity {
    name: "cliff-explosives",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CLUSTER_GRENADE: Entity = Entity {
    name: "cluster-grenade",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static COAL_PARTICLE: Entity = Entity {
    name: "coal-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static COPPER_ORE_PARTICLE: Entity = Entity {
    name: "copper-ore-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DECONSTRUCTIBLE_TILE_PROXY: Entity = Entity {
    name: "deconstructible-tile-proxy",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DEFENDER_CAPSULE: Entity = Entity {
    name: "defender-capsule",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DESTROYER_CAPSULE: Entity = Entity {
    name: "destroyer-capsule",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DISTRACTOR_ARTILLERY_PROJECTILE: Entity = Entity {
    name: "distractor-artillery-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DISTRACTOR_CAPSULE: Entity = Entity {
    name: "distractor-capsule",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static DUMMY_FLAME_THROWER_EXPLOSION: Entity = Entity {
    name: "dummy-flame-thrower-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ELECTRIC_BEAM: Entity = Entity {
    name: "electric-beam",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ELECTRIC_BEAM_NO_SOUND: Entity = Entity {
    name: "electric-beam-no-sound",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ELECTRIC_SPIT_PROJECTILE: Entity = Entity {
    name: "electric-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ENTITY_GHOST: Entity = Entity {
    name: "entity-ghost",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSION: Entity = Entity {
    name: "explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSION_GUNSHOT: Entity = Entity {
    name: "explosion-gunshot",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSION_GUNSHOT_SMALL: Entity = Entity {
    name: "explosion-gunshot-small",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSION_HIT: Entity = Entity {
    name: "explosion-hit",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSION_REMNANTS_PARTICLE: Entity = Entity {
    name: "explosion-remnants-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSIVE_ARTILLERY_PROJECTILE: Entity = Entity {
    name: "explosive-artillery-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSIVE_CANNON_PROJECTILE: Entity = Entity {
    name: "explosive-cannon-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSIVE_ROCKET: Entity = Entity {
    name: "explosive-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSIVE_SPIT_PROJECTILE: Entity = Entity {
    name: "explosive-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static EXPLOSIVE_URANIUM_CANNON_PROJECTILE: Entity = Entity {
    name: "explosive-uranium-cannon-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FAKE_SELECTION_BOX_2X2: Entity = Entity {
    name: "fake-selection-box-2x2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FIRE_FLAME: Entity = Entity {
    name: "fire-flame",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FIRE_FLAME_ON_TREE: Entity = Entity {
    name: "fire-flame-on-tree",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FIRE_SPIT_PROJECTILE: Entity = Entity {
    name: "fire-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FIRE_STICKER: Entity = Entity {
    name: "fire-sticker",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FLAMETHROWER_FIRE_STREAM: Entity = Entity {
    name: "flamethrower-fire-stream",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static FLYING_TEXT: Entity = Entity {
    name: "flying-text",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GARBALLO: Entity = Entity {
    name: "garballo",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GARBALLO_MINI_DRY: Entity = Entity {
    name: "garballo-mini-dry",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_ASTERISK: Entity = Entity {
    name: "green-asterisk",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_BUSH_MINI: Entity = Entity {
    name: "green-bush-mini",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_CARPET_GRASS: Entity = Entity {
    name: "green-carpet-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_CORAL_MINI: Entity = Entity {
    name: "green-coral-mini",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_HAIRY_GRASS: Entity = Entity {
    name: "green-hairy-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_PITA: Entity = Entity {
    name: "green-pita",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_PITA_MINI: Entity = Entity {
    name: "green-pita-mini",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GREEN_SMALL_GRASS: Entity = Entity {
    name: "green-small-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GRENADE: Entity = Entity {
    name: "grenade",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static GROUND_EXPLOSION: Entity = Entity {
    name: "ground-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static HANDHELD_FLAMETHROWER_FIRE_STREAM: Entity = Entity {
    name: "handheld-flamethrower-fire-stream",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static IRON_ORE_PARTICLE: Entity = Entity {
    name: "iron-ore-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ITEM_ON_GROUND: Entity = Entity {
    name: "item-on-ground",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ITEM_REQUEST_PROXY: Entity = Entity {
    name: "item-request-proxy",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LASER: Entity = Entity {
    name: "laser",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LASER_BUBBLE: Entity = Entity {
    name: "laser-bubble",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LEAF_PARTICLE: Entity = Entity {
    name: "leaf-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static LEVIATHAN_SPIT_PROJECTILE: Entity = Entity {
    name: "leviathan-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MASSIVE_EXPLOSION: Entity = Entity {
    name: "massive-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static MEDIUM_EXPLOSION: Entity = Entity {
    name: "medium-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ORANGE_ARROW_WITH_CIRCLE: Entity = Entity {
    name: "orange-arrow-with-circle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ORANGE_CORAL_MINI: Entity = Entity {
    name: "orange-coral-mini",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PIERCING_SHOTGUN_PELLET: Entity = Entity {
    name: "piercing-shotgun-pellet",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static PIERCING_SPIT_PROJECTILE: Entity = Entity {
    name: "piercing-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POISON_ARTILLERY_CLOUD: Entity = Entity {
    name: "poison-artillery-cloud",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POISON_ARTILLERY_PROJECTILE: Entity = Entity {
    name: "poison-artillery-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POISON_CAPSULE: Entity = Entity {
    name: "poison-capsule",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POISON_CLOUD: Entity = Entity {
    name: "poison-cloud",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static POISON_SPIT_PROJECTILE: Entity = Entity {
    name: "poison-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static RAILGUN_BEAM: Entity = Entity {
    name: "railgun-beam",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static RED_ASTERISK: Entity = Entity {
    name: "red-asterisk",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROCKET: Entity = Entity {
    name: "rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROCKET_SILO_ROCKET: Entity = Entity {
    name: "rocket-silo-rocket",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROCKET_SILO_ROCKET_SHADOW: Entity = Entity {
    name: "rocket-silo-rocket-shadow",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROOT_A: Entity = Entity {
    name: "root-A",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROOT_B: Entity = Entity {
    name: "root-B",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHELL_PARTICLE: Entity = Entity {
    name: "shell-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_ACID_PROJECTILE: Entity = Entity {
    name: "shotgun-acid-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_AP_PROJECTILE: Entity = Entity {
    name: "shotgun-ap-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_ELECTRIC_PROJECTILE: Entity = Entity {
    name: "shotgun-electric-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_EXPLOSIVE_PROJECTILE: Entity = Entity {
    name: "shotgun-explosive-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_FLAME_PROJECTILE: Entity = Entity {
    name: "shotgun-flame-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_PELLET: Entity = Entity {
    name: "shotgun-pellet",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_POISON_PROJECTILE: Entity = Entity {
    name: "shotgun-poison-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SHOTGUN_URANIUM_PROJECTILE: Entity = Entity {
    name: "shotgun-uranium-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SLOWDOWN_CAPSULE: Entity = Entity {
    name: "slowdown-capsule",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SLOWDOWN_STICKER: Entity = Entity {
    name: "slowdown-sticker",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_FIRE_CLOUD: Entity = Entity {
    name: "small-fire-cloud",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_POISON_CLOUD: Entity = Entity {
    name: "small-poison-cloud",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMALL_SHIP_WRECK_GRASS: Entity = Entity {
    name: "small-ship-wreck-grass",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static SMOKE_FOR_MIGRATION: Entity = Entity {
    name: "smoke-for-migration",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STONE_PARTICLE: Entity = Entity {
    name: "stone-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STUN_STICKER: Entity = Entity {
    name: "stun-sticker",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TANK_FLAMETHROWER_FIRE_STREAM: Entity = Entity {
    name: "tank-flamethrower-fire-stream",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TILE_GHOST: Entity = Entity {
    name: "tile-ghost",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TITAN_SPIT_PROJECTILE: Entity = Entity {
    name: "titan-spit-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static TUTORIAL_FLYING_TEXT: Entity = Entity {
    name: "tutorial-flying-text",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static URANIUM_CANNON_EXPLOSION: Entity = Entity {
    name: "uranium-cannon-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static URANIUM_CANNON_PROJECTILE: Entity = Entity {
    name: "uranium-cannon-projectile",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static URANIUM_CANNON_SHELL_EXPLOSION: Entity = Entity {
    name: "uranium-cannon-shell-explosion",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static WATER_SPLASH: Entity = Entity {
    name: "water-splash",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static WOODEN_PARTICLE: Entity = Entity {
    name: "wooden-particle",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_FISSURE: Entity = Entity {
    name: "angels-fissure",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_NATURAL_GAS: Entity = Entity {
    name: "angels-natural-gas",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE1: Entity = Entity {
    name: "angels-ore1",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE2: Entity = Entity {
    name: "angels-ore2",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE3: Entity = Entity {
    name: "angels-ore3",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE4: Entity = Entity {
    name: "angels-ore4",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE5: Entity = Entity {
    name: "angels-ore5",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ANGELS_ORE6: Entity = Entity {
    name: "angels-ore6",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static CRUDE_OIL: Entity = Entity {
    name: "crude-oil",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static IRON_ORE: Entity = Entity {
    name: "iron-ore",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static COPPER_ORE: Entity = Entity {
    name: "copper-ore",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static URANIUM_ORE: Entity = Entity {
    name: "uranium-ore",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static COAL: Entity = Entity {
    name: "coal",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static STONE: Entity = Entity {
    name: "stone",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static BLUE_CHEST: Entity = Entity {
    name: "blue-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static ROCK_CHEST: Entity = Entity {
    name: "rock-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};

static HIDDEN_ELECTRIC_ENERGY_INTERFACE: Entity = Entity {
    name: "hidden-electric-energy-interface",
    crafting_speed: None,
    energy: EntityEnergy::Electric {
        usage: 0.00000000000000000000,
        drain: 0.00000000000000000000,
    },
};

static RED_CHEST: Entity = Entity {
    name: "red-chest",
    crafting_speed: None,
    energy: EntityEnergy::None,
};
