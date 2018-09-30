local json = require "lib/json"

-- NOTE(mickvangelderen): Have to pull out the values explicitly https://forums.factorio.com/viewtopic.php?t=41900#p245982

function map(f, obj)
  if obj == nil then
    return nil
  end

  local out = {}
  for k,v in pairs(obj) do
    out[k] = f(v)
  end
  return out
end

function ref_by_name(value)
  if value == nil then return value end
  return {
    __reference = true,
    name = value.name
  }
end


-- https://lua-api.factorio.com/0.16.51/LuaGroup.html
function transform_LuaGroup(group)
  return {
    name = group.name,
    ["type"] = group.type,
    -- group = group.group, -- Only defined for sub groups.
    -- subgroups = -- No need to list all the subgroups.
    order_in_recipe = group.order_in_recipe,
    order = group.order,
    valid = group.valid
  }
end

function transform_LuaSubGroup(group)
  return {
    name = group.name,
    ["type"] = group.type,
    -- group = -- No need to print parent group.
    -- subgroups = -- Only defined for groups.
    -- order_in_recipe -- Only defined for groups.
    order = group.order,
    valid = group.valid
  }
end

-- https://lua-api.factorio.com/0.16.51/LuaRecipePrototype.htmlclass LuaRecipePrototype
function transform_LuaRecipePrototype(recipe)
  return {
    enabled = recipe.enabled,
    name = recipe.name,
    localised_name = recipe.localised_name,
    localised_description = recipe.localised_description,
    category = recipe.category,
    ingredients = recipe.ingredients,
    products = recipe.products,
    hidden = recipe.hidden,
    hidden_from_flow_stats = recipe.hidden_from_flow_stats,
    always_show_made_in = recipe.always_show_made_in,
    energy = recipe.energy,
    order = recipe.order,
    group = transform_LuaGroup(recipe.group),
    subgroup = transform_LuaSubGroup(recipe.subgroup),
    request_paste_multiplier = recipe.request_paste_multiplier,
    overload_multiplier = recipe.overload_multiplier,
    allow_as_intermediate = recipe.allow_as_intermediate,
    allow_intermediates = recipe.allow_intermediates,
    show_amount_in_title = recipe.show_amount_in_title,
    always_show_products = recipe.always_show_products,
    emissions_multiplier = recipe.emissions_multiplier,
    valid = recipe.valid
  }
end

function transform_LuaItemPrototype(value)
  if value == nil then return value end
  return {
    -- has_flag = value.has_flag,
    -- get_ammo_type = value.get_ammo_type,
    ["type"] = value.type,
    name = value.name,
    -- localised_name = value.localised_name,
    -- localised_description = value.localised_description,
    order = value.order,
    place_result = ref_by_name(value.place_result),
    place_as_equipment_result = ref_by_name(value.place_as_equipment_result),
    place_as_tile_result = ref_by_name(value.place_as_tile_result),
    stackable = value.stackable,
    default_request_amount = value.default_request_amount,
    stack_size = value.stack_size,
    fuel_category = value.fuel_category,
    burnt_result = ref_by_name(value.burnt_result),
    fuel_value = value.fuel_value,
    fuel_acceleration_multiplier = value.fuel_acceleration_multiplier,
    fuel_top_speed_multiplier = value.fuel_top_speed_multiplier,
    fuel_emissions_multiplier = value.fuel_emissions_multiplier,
    subgroup = transform_LuaSubGroup(value.subgroup),
    group = transform_LuaGroup(value.group),
    flags = value.flags,
    rocket_launch_products = value.rocket_launch_products,
    can_be_mod_opened = value.can_be_mod_opened,
    magazine_size = value.magazine_size,
    equipment_grid = ref_by_name(value.equipment_grid),
    resistances = value.resistances,
    inventory_size_bonus = value.inventory_size_bonus,
    capsule_action = value.capsule_action,
    attack_parameters = value.attack_parameters,
    inventory_size = value.inventory_size,
    item_filters = map(ref_by_name, value.item_filters),
    item_group_filters = map(transform_LuaGroup, value.item_group_filters),
    item_subgroup_filters = map(transform_LuaSubGroup, value.item_subgroup_filters),
    filter_mode = value.filter_mode,
    insertion_priority_mode = value.insertion_priority_mode,
    localised_filter_message = value.localised_filter_message,
    extend_inventory_by_default = value.extend_inventory_by_default,
    default_label_color = value.default_label_color,
    draw_label_for_cursor_render = value.draw_label_for_cursor_render,
    speed = value.speed,
    -- attack_result = value.attack_result,
    -- attack_range = value.attack_range,
    -- build_distance_bonus = value.build_distance_bonus,
    -- item_drop_distance_bonus = value.item_drop_distance_bonus,
    -- reach_distance_bonus = value.reach_distance_bonus,
    -- item_pickup_distance_bonus = value.item_pickup_distance_bonus,
    -- loot_pickup_distance_bonus = value.loot_pickup_distance_bonus,
    -- resource_reach_distance_bonus = value.resource_reach_distance_bonus,
    -- module_effects = value.module_effects,
    category = value.category,
    -- tier = value.tier,
    -- limitations = value.limitations,
    -- limitation_message_key = value.limitation_message_key,
    -- straight_rail = value.straight_rail,
    -- curved_rail = value.curved_rail,
    -- repair_result = value.repair_result,
    -- selection_border_color = value.selection_border_color,
    -- alt_selection_border_color = value.alt_selection_border_color,
    -- selection_mode_flags = value.selection_mode_flags,
    -- alt_selection_mode_flags = value.alt_selection_mode_flags,
    -- selection_cursor_box_type = value.selection_cursor_box_type,
    -- alt_selection_cursor_box_type = value.alt_selection_cursor_box_type,
    -- always_include_tiles = value.always_include_tiles,
    -- show_in_library = value.show_in_library,
    -- entity_filter_slots = value.entity_filter_slots,
    -- tile_filter_slots = value.tile_filter_slots,
    -- durability_description_key = value.durability_description_key,
    -- durability = value.durability,
    valid = value.valid
  }
end

function transform_LuaBurnerPrototype(value)
  if value == nil then return value end
  return {
    emissions = value.emissions,
    render_no_network_icon = value.render_no_network_icon,
    render_no_power_icon = value.render_no_power_icon,
    effectivity = value.effectivity,
    fuel_inventory_size = value.fuel_inventory_size,
    burnt_inventory_size = value.burnt_inventory_size,
    smoke = value.smoke,
    light_flicker = value.light_flicker,
    fuel_categories = value.fuel_categories,
    valid = value.valid
  }
end

function transform_LuaElectricEnergySourcePrototype(value)
  if value == nil then return value end
  return {
    buffer_capacity = value.buffer_capacity,
    usage_priority = value.usage_priority,
    input_flow_limit = value.input_flow_limit,
    output_flow_limit = value.output_flow_limit,
    drain = value.drain,
    emissions = value.emissions,
    render_no_network_icon = value.render_no_network_icon,
    render_no_power_icon = value.render_no_power_icon,
    valid = value.valid
  }
end

function transform_LuaEntityPrototype(value)
  if value == nil then return value end
  return {
    ["type"] = value.type,
    name = value.name,
    localised_name = value.localised_name,
    localised_description = value.localised_description,
    max_health = value.max_health,
    infinite_resource = value.infinite_resource,
    minimum_resource_amount = value.minimum_resource_amount,
    normal_resource_amount = value.normal_resource_amount,
    infinite_depletion_resource_amount = value.infinite_depletion_resource_amount,
    resource_category = value.resource_category,
    -- mineable_properties = value.mineable_properties,
    -- items_to_place_this = value.items_to_place_this,
    -- collision_box = value.collision_box,
    -- selection_box = value.selection_box,
    -- drawing_box = value.drawing_box,
    -- sticker_box = value.sticker_box,
    collision_mask = value.collision_mask,
    order = value.order,
    group = transform_LuaGroup(value.group),
    subgroup = transform_LuaSubGroup(value.subgroup),
    healing_per_tick = value.healing_per_tick,
    emissions_per_tick = value.emissions_per_tick,
    -- corpses = value.corpses,
    selectable_in_game = value.selectable_in_game,
    selection_priority = value.selection_priority,
    weight = value.weight,
    resistances = value.resistances,
    fast_replaceable_group = value.fast_replaceable_group,
    -- loot = value.loot,
    repair_speed_modifier = value.repair_speed_modifier,
    turret_range = value.turret_range,
    -- autoplace_specification = value.autoplace_specification,
    belt_speed = value.belt_speed,
    -- result_units = value.result_units,
    -- attack_result = value.attack_result,
    -- final_attack_result = value.final_attack_result,
    -- attack_parameters = value.attack_parameters,
    -- spawn_cooldown = value.spawn_cooldown,
    mining_drill_radius = value.mining_drill_radius,
    mining_speed = value.mining_speed,
    mining_power = value.mining_power,
    logistic_mode = value.logistic_mode,
    max_underground_distance = value.max_underground_distance,
    flags = value.flags,
    remains_when_mined = ref_by_name(value.remains_when_mined),
    additional_pastable_entities = ref_by_name(value.additional_pastable_entities),
    allow_copy_paste = value.allow_copy_paste,
    shooting_cursor_size = value.shooting_cursor_size,
    -- created_smoke = value.created_smoke,
    -- created_effect = value.created_effect,
    map_color = value.map_color,
    friendly_map_color = value.friendly_map_color,
    enemy_map_color = value.enemy_map_color,
    -- build_base_evolution_requirement = value.build_base_evolution_requirement,
    -- instruments = value.instruments,
    max_polyphony = value.max_polyphony,
    module_inventory_size = value.module_inventory_size,
    ingredient_count = value.ingredient_count,
    crafting_speed = value.crafting_speed,
    crafting_categories = value.crafting_categories,
    resource_categories = value.resource_categories,
    supply_area_distance = value.supply_area_distance,
    max_wire_distance = value.max_wire_distance,
    max_circuit_wire_distance = value.max_circuit_wire_distance,
    energy_usage = value.energy_usage,
    max_energy_usage = value.max_energy_usage,
    effectivity = value.effectivity,
    consumption = value.consumption,
    friction_force = value.friction_force,
    braking_force = value.braking_force,
    tank_driving = value.tank_driving,
    rotation_speed = value.rotation_speed,
    turret_rotation_speed = value.turret_rotation_speed,
    guns = map(ref_by_name, value.guns),
    speed = value.speed,
    speed_multiplier_when_out_of_energy = value.speed_multiplier_when_out_of_energy,
    max_payload_size = value.max_payload_size,
    energy_per_move = value.energy_per_move,
    energy_per_tick = value.energy_per_tick,
    max_energy = value.max_energy,
    min_to_charge = value.min_to_charge,
    max_to_charge = value.max_to_charge,
    burner_prototype = transform_LuaBurnerPrototype(value.burner_prototype),
    electric_energy_source_prototype = transform_LuaElectricEnergySourcePrototype(value.electric_energy_source_prototype),
    building_grid_bit_shift = value.building_grid_bit_shift,
    fluid_usage_per_tick = value.fluid_usage_per_tick,
    maximum_temperature = value.maximum_temperature,
    target_temperature = value.target_temperature,
    fluid = ref_by_name(value.fluid),
    fluid_capacity = value.fluid_capacity,
    pumping_speed = value.pumping_speed,
    stack = value.stack,
    allow_custom_vectors = value.allow_custom_vectors,
    count_as_rock_for_filtered_deconstruction = value.count_as_rock_for_filtered_deconstruction,
    filter_count = value.filter_count,
    production = value.production,
    time_to_live = value.time_to_live,
    distribution_effectivity = value.distribution_effectivity,
    explosion_beam = value.explosion_beam,
    explosion_rotate = value.explosion_rotate,
    tree_color_count = value.tree_color_count,
    alert_when_damaged = value.alert_when_damaged,
    alert_when_attacking = value.alert_when_attacking,
    color = value.color,
    collision_mask_collides_with_self = value.collision_mask_collides_with_self,
    collision_mask_collides_with_tiles_only = value.collision_mask_collides_with_tiles_only,
    collision_mask_considers_tile_transitions = value.collision_mask_considers_tile_transitions,
    allowed_effects = value.allowed_effects,
    rocket_parts_required = value.rocket_parts_required,
    fixed_recipe = value.fixed_recipe,
    construction_radius = value.construction_radius,
    logistic_radius = value.logistic_radius,
    energy_per_hit_point = value.energy_per_hit_point,
    create_ghost_on_death = value.create_ghost_on_death,
    timeout = value.timeout,
    running_speed = value.running_speed,
    maximum_corner_sliding_distance = value.maximum_corner_sliding_distance,
    build_distance = value.build_distance,
    drop_item_distance = value.drop_item_distance,
    reach_distance = value.reach_distance,
    reach_resource_distance = value.reach_resource_distance,
    item_pickup_distance = value.item_pickup_distance,
    loot_pickup_distance = value.loot_pickup_distance,
    enter_vehicle_distance = value.enter_vehicle_distance,
    ticks_to_keep_gun = value.ticks_to_keep_gun,
    ticks_to_keep_aiming_direction = value.ticks_to_keep_aiming_direction,
    ticks_to_stay_in_combat = value.ticks_to_stay_in_combat,
    respawn_time = value.respawn_time,
    damage_hit_tint = value.damage_hit_tint,
    character_corpse = ref_by_name(value.character_corpse),
    valid = value.valid
  }
end

function transform_LuaFluidPrototype(value)
  if value == nil then return value end
  return {
    name = value.name,
    -- localised_name = value.localised_name,
    -- localised_description = value.localised_description,
    default_temperature = value.default_temperature,
    max_temperature = value.max_temperature,
    heat_capacity = value.heat_capacity,
    pressure_to_speed_ratio = value.pressure_to_speed_ratio,
    flow_to_energy_ratio = value.flow_to_energy_ratio,
    max_push_amount = value.max_push_amount,
    ratio_to_push = value.ratio_to_push,
    order = value.order,
    group = transform_LuaGroup(value.group),
    subgroup = transform_LuaSubGroup(value.subgroup),
    base_color = value.base_color,
    flow_color = value.flow_color,
    gas_temperature = value.gas_temperature,
    emissions_multiplier = value.emissions_multiplier,
    fuel_value = value.fuel_value,
    valid = value.valid
  }
end

function on_export(event)
  game.write_file("items.json", json.encode(
                    map(transform_LuaItemPrototype, game.item_prototypes)
  ))

  game.write_file("fluids.json", json.encode(
                    map(transform_LuaFluidPrototype, game.fluid_prototypes)
  ))

  game.write_file("entities.json", json.encode(
                    map(transform_LuaEntityPrototype, game.entity_prototypes)
  ))

  game.write_file("recipes.json", json.encode(
                    map(transform_LuaRecipePrototype, game.recipe_prototypes)
  ))
end

commands.add_command("export", nil, on_export)
