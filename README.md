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
