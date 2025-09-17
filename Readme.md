# Nikolaj (Rust version)

## Intro
Rust version on Nikolaj (Python) SC2 AI.  
Made using rust-sc2 library.  
Idea is to make slow push Terran BOT same as in the python, but with less performance limitation.
## Structures

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


<summary><strong>Refinery</strong></summary>

| Task       | Status |
|------------|--------|
| Construct  | ✅     |


<summary><strong>Supply Depot</strong></summary>

| Task         | Status |
|--------------|--------|
| Construct    | ✅     |
| Open/Close   | ✅     |


<summary><strong>Barracks</strong></summary>

| Task             | Status |
|------------------|--------|
| Construct        | ✅     |
| Construct Addon  | ✅     |
| Train            | ✅     |
| Fly/Land         | ✅     |

<summary><strong>Factory</strong></summary>

| Task             | Status |
|------------------|--------|
| Construct        | ✅     |
| Construct Addon  | ✅     |
| Train            | ✅     |
| Fly/Land         | ✅     |

<summary><strong>Starport</strong></summary>

| Task             | Status |
|------------------|--------|
| Construct        | ✅     |
| Construct Addon  | ✅     |
| Train            | ✅     |
| Fly/Land         | ✅     |

<summary><strong>Engineering Bay</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |

<summary><strong>Armory</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |

<summary><strong>Fusion Core</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |

<summary><strong>Missile Turret</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |

<summary><strong>Bunker</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Control  | ⬜     |

<summary><strong>Ghost Academy</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |
| Upgrades | ⬜     |
| Nukes    | ⬜     |

<summary><strong>Sensor Tower</strong></summary>

| Task     | Status |
|----------|--------|
| Construct| ⬜     |

---

## CC Units

<summary><strong>SCV</strong></summary>

| Task                              | Status |
|-----------------------------------|--------|
| Distribution                      | ✅     |
| Speedmining                       | ✅     |
| Finish building without workers   | ✅     |
| Attack nearby enemy               | ⬜     |
| Repair friendly units             | ✅     |
| Repair buildings                  | ✅     |
| Ramp block answer                 | ⬜     |
| Worker rush answer                | ⬜     |
| Planetary Fortress rush answer    | ⬜     |

---

## Barracks Units

<summary><strong>Barracks Units</strong></summary>

| Unit    | Train | Control |
|---------|:-----:|:-------:|
| Marine  | ✅    | ⬜      |
| Marauder| ✅    | ⬜      |
| Reaper  | ✅    | ⬜      |
| Ghost   | ⬜    | ⬜      |


---

## Factory Units

<summary><strong>Factory Units</strong></summary>

| Unit       | Train | Control |
|------------|:-----:|:-------:|
| Hellion    | ⬜    | ⬜      |
| Siege Tank | ⬜    | ⬜      |
| Widow Mine | ⬜    | ⬜      |
| Cyclone    | ⬜    | ⬜      |
| Thor       | ⬜    | ⬜      |

---

## Starport Units

<summary><strong>Starport Units</strong></summary>

| Unit         | Train | Control | Harass | 
|--------------|:-----:|:-------:|:------:|
| Viking       | ⬜    | ⬜      |        |
| Medivac      | ⬜    | ⬜      | ⬜      |
| Banshee      | ⬜    | ⬜      | ⬜      |
| Raven        | ⬜    | ⬜      | ⬜      |
| Liberator    | ⬜    | ⬜      | ⬜      |
| Battlecruiser| ⬜    | ⬜      | ⬜      |

---

## Utilities

<summary><strong>Utilities</strong></summary>

| Feature             | Status |
|---------------------|--------|
| Building placement  | ✅     |
| Combat strategy     | ✅     |
| Unit counter        | ⬜     |
| Unit rally points   | ✅     |
| Walls               | ⬜     |
| Pathfinding         | ⬜     |