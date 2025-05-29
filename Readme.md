# Nikolaj (Rust version)

## Intro
Rust version on Nikolaj (Python) SC2 AI.  
Made using rust-sc2 library.  
Idea is to make slow push Terran BOT same as in the python, but with less performance limitation.
## Structures

<details>
<summary><strong>Command Center</strong></summary>

| Task                         | Status |
|-----------------------------|--------|
| Construct                   | ✅     |
| Fly/Land                    | ✅     |
| Morph                       | ✅     |
| Scan hidden enemies         | ✅     |
| Search for leftover bases   | ✅     |
| M.U.L.E. drop               | ✅     |
| Train SCVs                  | ✅     |
| Drop Emergency Depot        | ⬜     |

</details>

<details>
<summary><strong>Refinery</strong></summary>

| Task       | Status |
|------------|--------|
| Construct  | ✅     |

</details>

<details>
<summary><strong>Supply Depot</strong></summary>

| Task         | Status |
|--------------|--------|
| Construct    | ✅     |
| Open/Close   | ✅     |

</details>

<details>
<summary><strong>Barracks</strong></summary>

| Task             | Status |
|------------------|--------|
| Construct        | ✅     |
| Construct Addon  | ✅     |
| Train            | ✅     |
| Fly/Land         | ⬜     |

</details>

<details>
<summary><strong>Factory</strong></summary>

| Task             | Status |
|------------------|--------|
| Construct        | ✅     |
| Construct Addon  | ⬜     |
| Train            | ⬜     |
| Fly/Land         | ⬜     |

</details>

<details>
<summary><strong>Starport</strong></summary>

| Task             | Status |
|------------------|--------|
| Construct        | ⬜     |
| Construct Addon  | ⬜     |
| Train            | ⬜     |
| Fly/Land         | ⬜     |

</details>

<details>
<summary><strong>Engineering Bay</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |

</details>

<details>
<summary><strong>Armory</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |

</details>

<details>
<summary><strong>Fusion Core</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |

</details>

<details>
<summary><strong>Missile Turret</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |

</details>

<details>
<summary><strong>Bunker</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Control  | ⬜     |

</details>

<details>
<summary><strong>Ghost Academy</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |
| Nukes    | ⬜     |

</details>

<details>
<summary><strong>Sensor Tower</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |

</details>

---

## CC Units

<details>
<summary><strong>SCV</strong></summary>

| Task                              | Status |
|-----------------------------------|--------|
| Distribution                      | ✅     |
| Split on start                    | ⬜     |
| Speedmining                       | ⬜     |
| Finish building without workers   | ✅     |
| Attack nearby enemy               | ⬜     |
| Repair friendly units             | ⬜     |
| Repair buildings                  | ⬜     |
| Ramp block answer                 | ⬜     |
| Worker rush answer                | ⬜     |
| Planetary Fortress rush answer    | ⬜     |

</details>

---

## Barracks Units

<details>
<summary><strong>Barracks Units</strong></summary>

| Unit    | Train | Control |
|---------|:-----:|:-------:|
| Marine  | ✅    | ⬜      |
| Marauder| ✅    | ⬜      |
| Reaper  | ✅    | ⬜      |
| Ghost   | ⬜    | ⬜      |

</details>

---

## Factory Units

<details>
<summary><strong>Factory Units</strong></summary>

| Unit       | Train | Control |
|------------|:-----:|:-------:|
| Hellion    | ⬜    | ⬜      |
| Siege Tank | ⬜    | ⬜      |
| Widow Mine | ⬜    | ⬜      |
| Cyclone    | ⬜    | ⬜      |
| Thor       | ⬜    | ⬜      |

</details>

---

## Starport Units

<details>
<summary><strong>Starport Units</strong></summary>

| Unit         | Train | Control | Harass | 
|--------------|:-----:|:-------:|:------:|
| Viking       | ⬜    | ⬜      |        |
| Medivac      | ⬜    | ⬜      | ⬜      |
| Banshee      | ⬜    | ⬜      | ⬜      |
| Raven        | ⬜    | ⬜      | ⬜      |
| Liberator    | ⬜    | ⬜      | ⬜      |
| Battlecruiser| ⬜    | ⬜      | ⬜      |

</details>

---

## Utilities

<details>
<summary><strong>Utilities</strong></summary>

| Feature             | Status |
|---------------------|--------|
| Building placement  | ✅     |
| Unit rally points   | ⬜     |
| Walls               | ⬜     |
| Pathfinding         | ⬜     |

</details>
