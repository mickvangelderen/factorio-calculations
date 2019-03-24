#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Item {
    pub name: &'static str,
}

macro_rules! impl_items {
    ($(($Item:ident, $name:expr),)*) => {
        $(
            pub static $Item: Item = Item {
                name: $name,
            };
        )*
    }
}

impl_items!{
    (IRON_ORE, "Iron Ore"),
    (IRON_INGOT, "Iron Ingot"),
    (IRON_PLATE, "Iron Plate"),
    (REINFORCED_IRON_PLATE, "Reinforced Iron Plate"),
    (IRON_ROD, "Iron Rod"),
    (SCREW, "Screw"),
    (COPPER_ORE, "Copper Ore"),
    (COPPER_INGOT, "Copper Ingot"),
    (WIRE, "Wire"),
    (CABLE, "Cable"),
    (MODULAR_FRAME, "Modular Frame"),
    (ROTOR, "Rotor"),
    (STATOR, "Stator"),
    (MOTOR, "Motor"),
    (COAL_ORE, "Coal Ore"),
    (STEEL_INGOT, "Steel Ingot"),
    (STEEL_PIPE, "Steel Pipe"),
    (STEEL_BEAM, "Steel Beam"),
    (REINFORCED_STEEL_BEAM, "Reinforced Steel Beam"),
}
