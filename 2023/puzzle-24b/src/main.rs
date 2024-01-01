use std::fs;

use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input
        .lines()
        .map(|line| {
            let values = |data: &str| -> Vec<f64> {
                data.split(',')
                    .map(|x| x.trim())
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect()
            };
            let (point, speed) = line.split_once(" @ ").unwrap();
            let (point, speed) = (values(point), values(speed));

            (
                (point[0], point[1], point[2]),
                (speed[0], speed[1], speed[2]),
            )
        })
        .collect::<Vec<((f64, f64, f64), (f64, f64, f64))>>();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let r_x = Int::new_const(&ctx, "r_x");
    let r_y = Int::new_const(&ctx, "r_y");
    let r_z = Int::new_const(&ctx, "r_z");
    let r_v_x = Int::new_const(&ctx, "r_v_x");
    let r_v_y = Int::new_const(&ctx, "r_v_y");
    let r_v_z = Int::new_const(&ctx, "r_v_z");
    for (i, &((x, y, z), (v_x, v_y, v_z))) in data.iter().enumerate() {
        let t = Int::new_const(&ctx, format!("t{}", i));
        let h_x = Int::from_i64(&ctx, x as i64);
        let h_y = Int::from_i64(&ctx, y as i64);
        let h_z = Int::from_i64(&ctx, z as i64);
        let h_v_x = Int::from_i64(&ctx, v_x as i64);
        let h_v_y = Int::from_i64(&ctx, v_y as i64);
        let h_v_z = Int::from_i64(&ctx, v_z as i64);

        solver.assert(&(&r_x + &r_v_x * &t)._eq(&(h_x + &h_v_x * &t)));
        solver.assert(&(&r_y + &r_v_y * &t)._eq(&(h_y + &h_v_y * &t)));
        solver.assert(&(&r_z + &r_v_z * &t)._eq(&(h_z + &h_v_z * &t)));
    }

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = model.eval(&(r_x + r_y + r_z), true).unwrap();
            println!("{}", solution);
        }
        _ => {
            println!("no solution");
        }
    }
}
