Factorio is one giant puzzle. Especially when you use Bobs' and Angels' mods.
Someone made a mod pack called Seablock taking the idea of the old Minecraft
SkyBlock maps where you start with almost nothing and have to figure out a way
to expand or reach some place.

Calculating the proportions in which you need certain machines/buildings
performing certain processes is very tedious and error prone. It is easy to
forget to factor in the correct crafting speed of the machine you intend to use,
forget a - sign somewhere, read/write a number wrong.

After I finally realized I was just solving a system of linear equations I
started to try and solve the problem with linear algebra. It took me a couple of
days to discover how to represent recipes. Then it took a couple more days to
discover how to set the quantity of some (machine, process) combination and how
to set the desired amount of materials.

When I finally got this done and implemented in Rust I built this setup in
Factorio to check how close my expected results were to the production graph of
Factorio. 

![Seablock early power setup](media/seablock-early-power-setup.png)

Unfortunately, my carbon output was not as high as I expected. By printing out
the expected production and consumption per minute I could find which resources
were and which weren't being calculated correctly.

![Expected production and consumption](media/debugging-editor-screenshot.png)

![Production graph in game](media/debugging-game-screenshot.png)

![Photo of the two screenshots](media/debugging-photo.jpg)

I found out I had the crushed coal to coke recipe wrong. It shouldve been 2
crushed coal become 2 coke but I had it written down as 1 crushed coal becomes 2
coke.

So yeah, actually used basic linear algebra! Pretty cool :smile:.

```
Setup
        0.25 x water_pumping (offshore_pump)
        0.38 x burn_oxygen (flare_stack)
        0.50 x burn_hydrogen (flare_stack)
        5.00 x dirt_water_electrolysis (electrolyser_mk1)
        1.67 x stone_crushing (ore_crusher_mk1)
        0.33 x water_mineralization (liquifier_mk1)
       10.00 x green_algae_growing (algae_farm_mk1)
        4.00 x green_algae_to_fiber (liquifier_mk1)
        6.67 x fiber_to_wood_pellet (assembly_machine_mk1)
        0.83 x wood_pellet_to_wood_brick (assembly_machine_mk1)
        2.92 x wood_brick_to_coal (stone_furnace_burning_carbon)
        2.16 x coal_to_carbon_dioxide (liquifier_mk1)
        0.59 x coal_to_crushed_coal (ore_crusher_mk1)
        0.88 x burn_crushed_coal_to_coke (stone_furnace_burning_carbon)
        1.18 x coke_to_carbon (liquifier_mk1)
        4.22 x boiler_mk1_carbon_to_power (boiler_mk1_burning_carbon)

Balance
        0.00 Carbon/s
        0.00 CarbonDioxide/s
        0.00 Coal/s
        0.00 Coke/s
        0.00 CrushedCoal/s
        0.00 CrushedStone/s
        0.00 Fiber/s
        0.00 GreenAlgae/s
        0.00 Hydrogen/s
  2784691.67 Joule/s
        0.00 MineralizedWater/s
        0.00 Oxygen/s
        0.00 Slag/s
        0.00 Water/s
        0.00 WoodBrick/s
        0.00 WoodPellet/s

  Production  Consumption
      158.82      -158.82 Carbon/m
     4852.94     -4852.94 CarbonDioxide/m
      150.00      -150.00 Coal/m
      105.88      -105.88 Coke/m
      105.88      -105.88 CrushedCoal/m
      300.00      -300.00 CrushedStone/m
      600.00      -600.00 Fiber/m
     1200.00     -1200.00 GreenAlgae/m
     6000.00     -6000.00 Hydrogen/m
455955882.35 -288874382.35 Joule/m
     3000.00     -3000.00 MineralizedWater/m
     4500.00     -4500.00 Oxygen/m
      150.00      -150.00 Slag/m
    18000.00    -18000.00 Water/m
       50.00       -50.00 WoodBrick/m
      100.00      -100.00 WoodPellet/m

Setup
        0.28 x water_pumping (offshore_pump)
        1.00 x pump_viscous_mud_water (seafloor_pump)
       10.00 x wash_viscous_mud_water (washing_plant_mk1)
       25.00 x heavy_mud_water_to_nodule (washing_plant_mk1)

Balance
        0.00 HeavyMudWater/s
 -3615500.00 Joule/s
        3.00 Mud/s
        7.50 Nodule/s
        0.00 ViscousMudWater/s
        0.00 Water/s

  Production  Consumption
    18000.00    -18000.00 HeavyMudWater/m
        0.00 -216930000.00 Joule/m
      180.00         0.00 Mud/m
      450.00         0.00 Nodule/m
    18000.00    -18000.00 ViscousMudWater/m
    20250.00    -20250.00 Water/m

```
