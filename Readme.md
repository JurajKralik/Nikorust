# Nikolaj (Rust version)

## Intro
Rust version of Nikolaj (Python) SC2 AI.  

Made with [rust-sc2 library](https://github.com/UltraMachine/rust-sc2)

Idea is to make slow push Terran BOT same as in the python, but with less performance limitation.

Copying of Nikolaj's parts is permitted, but please follow the [guidelines](https://aiarena.net/wiki/bot-development/getting-started/#wiki-toc-using-existing-bots-as-a-reference-or-starting-point).


<details> <summary><strong>🏗️ Structures</strong></summary>

| Structure           | Task / Feature            | Status |
| ------------------- | ------------------------- | :----: |
| **Command Center**  | Construct                 |    ✅   |
|                     | Fly / Land                |    ✅   |
|                     | CC Morph (Orbital / PF)   |    ✅   |
|                     | Scan Hidden Enemies       |    ✅   |
|                     | Search for Leftover Bases |    ✅   |
|                     | M.U.L.E. Drop             |    ✅   |
|                     | Train SCVs                |    ✅   |
|                     | Emergency Depot Drop      |    ⬜   |
| **Refinery**        | Construct                 |    ✅   |
| **Supply Depot**    | Construct                 |    ✅   |
|                     | Depots Open / Close       |    ✅   |
| **Barracks**        | Construct                 |    ✅   |
|                     | Construct Addon           |    ✅   |
|                     | Train Units               |    ✅   |
|                     | Fly / Land                |    ✅   |
| **Factory**         | Construct                 |    ✅   |
|                     | Construct Addon           |    ✅   |
|                     | Train Units               |    ✅   |
|                     | Fly / Land                |    ✅   |
| **Starport**        | Construct                 |    ✅   |
|                     | Construct Addon           |    ✅   |
|                     | Train Units               |    ✅   |
|                     | Fly / Land                |    ✅   |
| **Bunker**          | Construct                 |    ⬜   |
|                     | Load / Unload Control     |    ⬜   |
| **Engineering Bay** | Construct                 |    ⬜   |
|                     | Research Upgrades         |    ⬜   |
| **Armory**          | Construct                 |    ⬜   |
|                     | Research Upgrades         |    ⬜   |
| **Fusion Core**     | Construct                 |    ⬜   |
|                     | Research Upgrades         |    ⬜   |
| **Missile Turret**  | Construct                 |    ⬜   |
| **Ghost Academy**   | Construct                 |    ⬜   |
|                     | Research Upgrades         |    ⬜   |
|                     | Nuke Production / Launch  |    ⬜   |
| **Sensor Tower**    | Construct                 |    ⬜   |

</details>
<details> <summary><strong>👷‍♂️ SCV</strong></summary>

| Task                            | Status |
| ------------------------------- | :----: |
| Distribution                    |    ✅   |
| Speedmining                     |    ✅   |
| Finish building without workers |    ✅   |
| Attack nearby enemy             |    ⬜   |
| Repair friendly units           |    ✅   |
| Repair buildings                |    ✅   |
| Ramp block response             |    ⬜   |
| Worker rush response            |    ⬜   |
| Planetary Fortress rush answer  |    ⬜   |

</details>

<details> <summary><strong>🪖 Barracks Units</strong></summary>

| Unit     | Train | Control |
| -------- | :---: | :-----: |
| Marine   |   ✅   |    ✅    |
| Marauder |   ✅   |    ✅    |
| Reaper   |   ✅   |    ✅    |
| Ghost    |   ⬜   |    ⬜    |

</details>

<details> <summary><strong>⚙️ Factory Units</strong></summary>

| Unit       | Train | Control |
| ---------- | :---: | :-----: |
| Hellion    |   ⬜   |    ⬜    |
| Siege Tank |   ✅   |    ✅    |
| Widow Mine |   ✅   |    ✅    |
| Cyclone    |   ✅   |    ⬜    |
| Thor       |   ✅   |    ⬜    |

</details>

<details> <summary><strong>🚀Starport Units</strong></summary>

| Unit          | Train | Control | Harass |
| ------------- | :---: | :-----: | :----: |
| Viking        |   ✅   |    ⬜    |        |
| Medivac       |   ✅   |    ✅    |    ⬜   |
| Banshee       |   ✅   |    ✅    |    ✅   |
| Raven         |   ✅   |    ✅    |    ⬜   |
| Liberator     |   ⬜   |    ⬜    |    ⬜   |
| Battlecruiser |   ⬜   |    ⬜    |    ⬜   |

</details>

<details> <summary><strong>🧰Utilities</strong></summary>

| Feature            | Status |
| ------------------ | :----: |
| Building placement |    ✅   |
| Combat strategy    |    ✅   |
| Unit counter       |    ⬜   |
| Unit rally points  |    ✅   |
| Walls              |    ⬜   |
| Heatmaps           |    ⬜   |
| Pathfinding        |    ⬜   |

</details>