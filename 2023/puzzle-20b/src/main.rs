use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use num::integer::lcm;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let module_configuration = module_configuration(&input);
    let mut module_parents = module_parents(module_configuration.clone());
    let mut flip_flop = HashMap::new();

    let rx_parent = module_configuration
        .iter()
        .filter(|(_, (_, to))| to.contains(&"rx".to_string()))
        .map(|(module, _)| module.clone())
        .next()
        .unwrap();
    let binding = module_parents.clone();
    let mut rx_parents = binding.get(&rx_parent).unwrap().clone();

    let mut i = 0;
    loop {
        i += 1;
        press_button(
            &module_configuration,
            &mut module_parents,
            &mut flip_flop,
            rx_parent.clone(),
            &mut rx_parents,
            i,
        );
        if rx_parents.values().all(|x| x != &0) {
            break;
        }
    }

    let mut total = 1;
    for &value in rx_parents.values() {
        total = lcm(total, value);
    }

    println!("{}", total);
}

fn module_configuration(input: &str) -> HashMap<String, (u128, Vec<String>)> {
    let mut configuration = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let (module_type, module) = if from.contains('%') {
            (1, &from[1..])
        } else if from.contains('&') {
            (2, &from[1..])
        } else {
            (0, from)
        };
        let to = to
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        configuration.insert(module.to_string(), (module_type, to));
    }

    configuration
}

fn press_button(
    module_configuration: &HashMap<String, (u128, Vec<String>)>,
    module_parents: &mut HashMap<String, HashMap<String, u128>>,
    flip_flop: &mut HashMap<String, u128>,
    rx_parent: String,
    rx_parents: &mut HashMap<String, u128>,
    i: u128,
) {
    let mut queue = VecDeque::new();
    queue.push_back((0, "".to_string(), "broadcaster".to_string()));
    while !queue.is_empty() {
        let (pulse, parent_module, module) = queue.pop_front().unwrap();
        if pulse == 1 && rx_parents.contains_key(&parent_module) && module == rx_parent {
            if rx_parents.get(&parent_module).unwrap() == &0 {
                rx_parents.insert(parent_module.clone(), i);
            }
        }
        if module_configuration.get(&module).is_none() {
            continue;
        }
        let (module_type, to) = module_configuration.get(&module).unwrap();
        if *module_type == 0 {
            for to in to.iter() {
                queue.push_back((pulse, module.clone(), to.clone()));
            }
        } else if *module_type == 1 {
            if pulse == 1 {
                continue;
            }

            if let Some(mode) = flip_flop.get(&module) {
                flip_flop.insert(module.clone(), (mode + 1) % 2);
            } else {
                flip_flop.insert(module.clone(), 1);
            }

            let current_mode = flip_flop[&module];
            let pulse = current_mode;
            for to in to.iter() {
                queue.push_back((pulse, module.clone(), to.clone()));
            }
        } else {
            module_parents
                .get_mut(&module)
                .unwrap()
                .insert(parent_module.clone(), pulse);
            let pulse = !module_parents
                .get(&module)
                .unwrap()
                .values()
                .all(|v| v == &1) as u128;
            for to in to.iter() {
                queue.push_back((pulse, module.clone(), to.clone()));
            }
        }
    }
}

fn module_parents(
    module_configuration: HashMap<String, (u128, Vec<String>)>,
) -> HashMap<String, HashMap<String, u128>> {
    let conjunction_modules = module_configuration
        .iter()
        .filter(|(_, value)| value.0 == 2)
        .map(|(key, _)| key)
        .collect::<Vec<&String>>();

    let mut conjunction = HashMap::<String, HashMap<String, u128>>::new();
    for (parent, (_, to)) in module_configuration.iter() {
        for &module in conjunction_modules.iter() {
            if to.contains(module) {
                if let Some(data) = conjunction.get_mut(module) {
                    data.insert(parent.to_string(), 0);
                } else {
                    conjunction.insert(module.clone(), HashMap::<String, u128>::new());
                    conjunction
                        .get_mut(module)
                        .unwrap()
                        .insert(parent.to_string(), 0);
                }
            }
        }
    }

    conjunction
}
