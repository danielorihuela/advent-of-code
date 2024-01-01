use std::{
    collections::{HashMap, HashSet},
    fs,
};

use rand::Rng;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        nodes.insert(from.to_string());

        let to = to.split(' ').collect::<Vec<&str>>();
        for to in to {
            nodes.insert(to.to_string());
            edges.insert((from.to_string(), to.to_string()));
        }
    }

    let mut numeric_nodes = HashMap::new();
    for i in 0..nodes.len() {
        numeric_nodes.insert(nodes.iter().nth(i).unwrap().clone(), i);
    }
    let mut numeric_edges = vec![];
    for (u, v) in edges.iter() {
        numeric_edges.push((
            numeric_nodes.get(u).unwrap().clone(),
            numeric_nodes.get(v).unwrap().clone(),
        ));
    }

    let mut rng = rand::thread_rng();
    let mut counter = HashMap::new();
    while counter
        .iter()
        .filter(|((a, b), _)| a > &100 && b > &100)
        .count()
        == 0
    {
        let mut nnodes = numeric_nodes.clone();
        let mut eedges = numeric_edges.clone();
        while nnodes.len() != 2 {
            let r = rng.gen_range(0..eedges.len());

            let edge = eedges.remove(r);
            let u = nnodes
                .clone()
                .into_iter()
                .filter(|(_, b)| b == &edge.0)
                .next()
                .unwrap();
            let v = nnodes
                .clone()
                .into_iter()
                .filter(|(_, b)| b == &edge.1)
                .next()
                .unwrap();

            nnodes.remove(&u.0);
            nnodes.remove(&v.0);
            nnodes.insert(format!("{}_{}", u.0, v.0), u.1);

            let mut to_remove = vec![];
            let mut to_push = vec![];
            for (i, _) in eedges.clone().iter().enumerate() {
                if (eedges[i].0 == u.1 || eedges[i].0 == v.1)
                    && (eedges[i].1 == u.1 || eedges[i].1 == v.1)
                {
                    to_remove.push(i);
                } else if eedges[i].0 == u.1 || eedges[i].0 == v.1 {
                    to_remove.push(i);
                    to_push.push((u.1, eedges[i].1));
                } else if eedges[i].1 == u.1 || eedges[i].1 == v.1 {
                    to_remove.push(i);
                    to_push.push((eedges[i].0, u.1));
                }
            }
            let mut j = 0;
            for &value in to_remove.iter() {
                eedges.remove(value - j);
                j += 1;
            }
            for &value in to_push.iter() {
                eedges.push(value);
            }
        }

        if nnodes.len() == 2 {
            let a = nnodes.iter().next().unwrap().0.split('_').count();
            let b = nnodes.iter().nth(1).unwrap().0.split('_').count();
            *counter.entry((a, b)).or_insert(0) += 1;
        }
    }

    let counter = counter
        .iter()
        .filter(|((a, b), _)| a > &100 && b > &100)
        .collect::<Vec<_>>();
    let (a, b) = counter.iter().next().unwrap().0;
    println!("{}", a * b);
}
