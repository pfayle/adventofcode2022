use std::collections::HashMap;
use std::io;
use std::io::Read;
use strum::{IntoEnumIterator, EnumCount};
use strum_macros::{EnumCount, Display, EnumIter};

const TIME1: usize = 24;
const TIME2: usize = 32;
const USABLE_BLUEPRINTS: usize = 3;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let blueprints = parse_input(&buf);
    let mut quality = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let max = max_geodes_and_path(blueprint, TIME1, &Inventory::new(), 0);
        quality += max.0 * (i+1);
    }
    println!("Total quality: {quality}");
    let mut product = 1;
    blueprints.iter().take(USABLE_BLUEPRINTS).map(
        |b| max_geodes_and_path(b, TIME2, &Inventory::new(), 0)
    ).for_each(
        |(n, _)| {
            product *= n;
        }
    );
    println!("Max product: {product}");
}

#[derive(EnumCount, EnumIter, Hash, PartialEq, Eq, Clone, Copy, Display, Debug)]
enum Material {
    #[strum(to_string="ore")]
    Ore,
    #[strum(to_string="clay")]
    Clay,
    #[strum(to_string="obsidian")]
    Obsidian,
    #[strum(to_string="geode")]
    Geode,
}

use Material::*;

#[derive(Debug)]
struct Cost(HashMap<Material, usize>);

#[derive(Debug)]
struct Blueprint {
    costs: HashMap<Material, Cost>,
}

impl Blueprint {
    fn max(&self, material: &Material) -> Option<usize> {
        self.costs.values().map(|c| c.0.get(material))
            .filter(|c| c.is_some())
            .map(|c| *c.unwrap())
            .max()
    }
}

#[derive(Debug, Clone)]
struct Inventory{
    materials: HashMap<Material, usize>,
    robots: HashMap<Material, usize>,
}

impl Inventory {
    fn new() -> Self {
        let materials = HashMap::from([
            (Ore, 0),
            (Clay, 0),
            (Obsidian, 0),
            (Geode, 0),
        ]);
        let robots = HashMap::from([
            (Ore, 1),
            (Clay, 0),
            (Obsidian, 0),
            (Geode, 0),
        ]);
        Self{materials, robots}
    }

    fn can_afford(&self, cost: &Cost) -> bool {
        for (m, c) in &cost.0 {
            if *self.materials.get(m).unwrap() < *c {
                return false;
            }
        }
        true
    }

    fn buy(&mut self, cost: &Cost) {
        if !self.can_afford(cost) {
            panic!();
        }
        for (m, c) in &cost.0 {
            *self.materials.get_mut(m).unwrap() -= *c;
        }
    }

    fn gather(&mut self) {
        for (robot, num) in &self.robots {
            *self.materials.get_mut(robot).unwrap() += num;
        }
    }

    fn build(&mut self, material: &Material) {
        *self.robots.get_mut(material).unwrap() += 1;
    }
}

struct PurchaseOrder<'a> {
    order: &'a mut dyn Iterator<Item=&'a Material>,
}

impl<'a> PurchaseOrder<'a> {
    fn process(&mut self, blueprint: &Blueprint, time: usize, inventory: &mut Inventory) {
        let mut robot_material = self.order.next().unwrap();
        for _ in 0..time {
            self.tick(blueprint, inventory, &mut robot_material);
        }
    }

    fn tick(&mut self, blueprint: &Blueprint, inventory: &mut Inventory, robot_material: &mut &'a Material) -> bool {
        // pay
        let cost = blueprint.costs.get(robot_material).unwrap();
        let building = inventory.can_afford(cost);
        if building {
            inventory.buy(cost);
        }

        // gather
        inventory.gather();

        // build
        if building {
            inventory.build(robot_material);
            *robot_material = self.order.next().unwrap();
        }
        building
    }

    fn ticks_till_build(&mut self, blueprint: &Blueprint, time: usize, inventory: &mut Inventory, robot_material: &mut &'a Material) -> Option<usize> {
        if time == 0 {
            return None;
        }
        let mut ret = 0;
        loop {
            let ticks = self.tick(blueprint, inventory, robot_material);
            ret += 1;
            if ret >= time {
                return None;
            }
            if ticks {
                break;
            }
        }
        Some(ret)
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    for section in input.replace([' ', '\n'], " ").split("Blueprint").skip(1) {
        let mut costs = HashMap::new();
        for line in section.split("Each ").skip(1) {
            let mut words = line.split(' ');
            let material = words.next().unwrap();
            let mut cost = words.skip(2);
            for mat in Material::iter() {
                if mat.to_string() == material {
                    costs.insert(mat, parse_cost(&mut cost));
                }
            }
        }
        blueprints.push(Blueprint{costs});
    }
    blueprints
}

fn parse_cost(input: &mut dyn Iterator<Item=&str>) -> Cost {
    let mut cost = HashMap::new();
    while let Some(count) = input.next() {
        if let Ok(amount) = count.parse::<usize>() {
            let mat = input.next().unwrap().replace('.', "");
            let material = Material::iter().find(|m| m.to_string() == mat).unwrap();
            input.next();
            cost.insert(material, amount);
        }
    }
    Cost(cost)
}

fn max_geodes_and_path(blueprint: &Blueprint, time: usize, inventory: &Inventory, current_best: usize) -> (usize, Vec<Material>) {
    let mut best = current_best;
    let mut best_route = vec![];
    let g_robots = *inventory.robots.get(&Geode).unwrap();
    let geodes = *inventory.materials.get(&Geode).unwrap();
    if time == 0 {
        return (geodes, vec![]);
    }
    let lb = *lower_bound(blueprint, inventory, time).materials.get(&Geode).unwrap();
    if best < lb {
        best = lb;
    }
    let ub = geodes + g_robots * time + time * (time+1) / 2;
    let ub2 = *upper_bounds(blueprint, inventory, time).materials.get(&Geode).unwrap();
    if best >= ub || best >= ub2 {
        return (best, vec![]);
    }
    for robot_material in Material::iter() {
        // early continue if we don't need more of these.
        if let Some(robot_max) = blueprint.max(&robot_material) {
            if *inventory.robots.get(&robot_material).unwrap() >= robot_max {
                continue;
            }
        }
        let order = [robot_material];
        let mut route = PurchaseOrder{
            order: &mut order.iter().cycle(),
        };
        let mut inventory2 = inventory.clone();
        if let Some(ticks) = route.ticks_till_build(blueprint, time, &mut inventory2, &mut &robot_material) {
            let mut sub = max_geodes_and_path(blueprint, time - ticks, &inventory2, best);
            if best < sub.0 {
                best = sub.0;
                best_route = vec![robot_material];
                best_route.append(&mut sub.1);
            }
        } else {
            let geodes = *inventory2.materials.get(&Geode).unwrap();
            if best < geodes {
                best = geodes;
                best_route = vec![];
            }
        }
    }
    (best, best_route)
}

/// build robots in material order
fn lower_bound(blueprint: &Blueprint, inventory: &Inventory, time: usize) -> Inventory {
    let mut robot_order = vec![];
    for material in Material::iter().take(Material::COUNT - 1) {
        let current = *inventory.robots.get(&material).unwrap();
        let max = blueprint.max(&material).unwrap();
        for _ in (current + 1)..=max {
            robot_order.push(material);
        }
    }
    let mut ret = inventory.clone();
    PurchaseOrder{order: &mut robot_order.iter().chain([Geode].iter().cycle())}
        .process(blueprint, time, &mut ret);
    ret
}

/// build all affordable robots without paying
fn upper_bounds(blueprint: &Blueprint, inventory: &Inventory, time: usize) -> Inventory {
    if time == 0 {
        return inventory.clone();
    }
    let mut next_inventory = inventory.clone();
    next_inventory.gather();
    for material in Material::iter() {
        if inventory.can_afford(blueprint.costs.get(&material).unwrap()) {
            next_inventory.build(&material);
        }
    }
    upper_bounds(blueprint, &next_inventory, time - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_geodes_in_up_to_3_minutes() {
        let blueprint = Blueprint{
            costs: HashMap::from([
                (Ore, Cost(HashMap::from([
                    (Ore, 2),
                ]))),
                (Clay, Cost(HashMap::from([
                    (Ore, 3),
                ]))),
                (Obsidian, Cost(HashMap::from([
                    (Ore, 3), (Clay, 8),
                ]))),
                (Geode, Cost(HashMap::from([
                    (Ore, 3), (Obsidian, 12),
                ]))),
            ])
        };
        let mut inventory = Inventory::new();
        inventory.robots.insert(Geode, 3);
        let result = max_geodes_and_path(&blueprint, 1, &inventory, 0);
        assert_eq!(result.0, 3);
        assert_eq!(result.1.len(), 0);
        let mut inventory = Inventory::new();
        inventory.robots.insert(Geode, 3);
        let result = max_geodes_and_path(&blueprint, 2, &inventory, 0);
        assert_eq!(result.0, 6);
        assert_eq!(result.1.len(), 0);
        let mut inventory = Inventory::new();
        inventory.robots.insert(Geode, 3);
        let result = max_geodes_and_path(&blueprint, 3, &inventory, 0);
        assert_eq!(result.0, 9);
        assert_eq!(result.1.len(), 0);
    }

    #[test]
    fn max_geodes_in_six_mins() {
        let blueprint = Blueprint{
            costs: HashMap::from([
                (Ore, Cost(HashMap::from([
                    (Ore, 2),
                ]))),
                (Clay, Cost(HashMap::from([
                    (Ore, 3),
                ]))),
                (Obsidian, Cost(HashMap::from([
                    (Ore, 3), (Clay, 8),
                ]))),
                (Geode, Cost(HashMap::from([
                    (Ore, 3), (Obsidian, 12),
                ]))),
            ])
        };
        let mut inventory = Inventory::new();
        inventory.robots.insert(Geode, 3);
        let result = max_geodes_and_path(&blueprint, 6, &inventory, 0);
        assert_eq!(result.0, 18);
    }
}