use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let module_configuration = module_configuration(&input);
    let mut module_parents = module_parents(module_configuration.clone());
    let mut flip_flop = HashMap::new();

    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;
    for _ in 0..1000 {
        let (low_pulses, high_pulses) =
            press_button(&module_configuration, &mut module_parents, &mut flip_flop);
        total_low_pulses += low_pulses;
        total_high_pulses += high_pulses;
    }
    println!("{}", total_high_pulses * total_low_pulses);
}

fn module_configuration(input: &str) -> HashMap<String, (usize, Vec<String>)> {
    let mut configuration = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let (module_type, module) = if from.contains("%") {
            (1, &from[1..])
        } else if from.contains("&") {
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
    module_configuration: &HashMap<String, (usize, Vec<String>)>,
    module_parents: &mut HashMap<String, HashMap<String, usize>>,
    flip_flop: &mut HashMap<String, usize>,
) -> (usize, usize) {
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, "".to_string(), "broadcaster".to_string()));
    while !queue.is_empty() {
        let (pulse, parent_module, module) = queue.pop_front().unwrap();
        if pulse == 1 {
            high_pulses += 1;
        } else {
            low_pulses += 1;
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
                .all(|v| v == &1) as usize;
            for to in to.iter() {
                queue.push_back((pulse, module.clone(), to.clone()));
            }
        }
    }

    (low_pulses, high_pulses)
}

fn module_parents(
    module_configuration: HashMap<String, (usize, Vec<String>)>,
) -> HashMap<String, HashMap<String, usize>> {
    let conjunction_modules = module_configuration
        .iter()
        .filter(|(_, value)| value.0 == 2)
        .map(|(key, _)| key)
        .collect::<Vec<&String>>();

    let mut conjunction = HashMap::<String, HashMap<String, usize>>::new();
    for (parent, (_, to)) in module_configuration.iter() {
        for &module in conjunction_modules.iter() {
            if to.contains(&module) {
                if let Some(data) = conjunction.get_mut(module) {
                    data.insert(parent.to_string(), 0);
                } else {
                    conjunction.insert(module.clone(), HashMap::<String, usize>::new());
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
