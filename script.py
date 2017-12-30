from sympy import *

# resources
mineralized_water = symbols('mineralized_water')
water = symbols('water')
crushed_stone = symbols('crushed_stone')
green_algae = symbols('green_algae')
carbon_dioxide = symbols('carbon_dioxide')

# machines
liquifier_mk1_crafting_speed = 1.5
algae_farm_mk1_crafting_speed = 1

# recipes
mineralized_water_recipe = (100*water + 10*crushed_stone)/100
mineralized_water_recipe_crafting_time = 1

green_algae_recipe = (100*mineralized_water + 100*carbon_dioxide)/40
green_algae_recipe_crafting_time = 20

# recipe instances
mineralized_water_per_second = mineralized_water_recipe/mineralized_water_recipe_crafting_time*liquifier_mk1_crafting_speed
green_algae_per_second = green_algae_recipe/green_algae_recipe_crafting_time*algae_farm_mk1_crafting_speed

print(mineralized_water_per_second)
print(green_algae_per_second)
