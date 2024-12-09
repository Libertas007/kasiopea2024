use std::collections::{HashMap, VecDeque};
use graph::prelude::*;
use num_integer::Roots;

use std::{fs, slice, thread};
use std::str;
use std::sync::mpsc;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("F.txt").unwrap();

    let problems: usize = content.lines().next().unwrap().parse().unwrap();

    //fs::write("F-outB.txt", output2.trim()).unwrap();

    /*let (tx, rx) = mpsc::channel();
    let n_blocks = 16;
    let block = problems / n_blocks;

    for i in 0..n_blocks {
        let mut problem_count = block;

        if i + 1 == n_blocks {
            problem_count += problems % n_blocks;
        }

        let sender = tx.clone();
        let padding = block * i + 1;

        //println!("{} {} {}", i, problem_count, padding);

        let contenta = content.clone();
        thread::spawn(move || {
            sender.send((i, solve(contenta, problem_count, padding))).unwrap();
        });
    }

    let mut output = vec![String::new(); n_blocks];

    let mut recieved = 0;

    for msg in rx {
        output[msg.0] = msg.1.clone();
        recieved += 1;
        if recieved == n_blocks {
            break
        }
        //println!("{:?}", msg);
    }*/

    //solve(content.clone(), problems, 1);
    let output = solve_exp(content.clone(), problems, 1);

    println!("{:?}", start.elapsed());
    fs::write("F-out.txt", output.trim()).unwrap();
}

fn solve_exp(content: String, problems: usize, padding: usize) -> String {
    let content: Vec<&str> = content.lines().collect();
    let mut line_ptr: usize = padding;

    let mut output: String = String::new();

    for _i in 0..problems {
        let values: Vec<i128> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 1;

        let D = values[0];
        let K = values[1];
        let C = values[2];
        let L = values[3];

        /*let mut stored = 0;
        let mut production = 0;*/
        /*let mut houses = 0;*/

        let a = -K;
        let b = (-K + 2*K*D + 2*C*L);
        let c = -2*C*L*D;

        let d = (b*b - 4*a*c);

        let x1: f64 = (-b + d.sqrt()) as f64 / (2*a) as f64;
        let x2: f64 = (2*D - 1) as f64 / 2.0;

        let mut x = x1;

        if x2 < x1 {
            x = x2;
        }

        let mut expected = -1;

            expected = simulate(D, K, C, L);

        /*let houses = ((((K as f64)*(x*x - x) / 2.0) +
            (K as f64)*x*((D as f64) - x))
            / (C as f64)).floor();
        let limit = L as f64 * (D as f64- x);

        let res = houses.min(limit);*/

        let mut houses = 0;

        for i in (x.floor() as i128 - 10).max(0)..D {
            let D_left = D - i;

            let a = (((K * (i * i - i) / 2) + K * i * D_left) as f64 / (C) as f64).floor() as i128;
            let b = L * D_left;

            let min = a.min(b);


            if min < houses {
                break;
            }

            houses = min;
        }


        if expected != -1 && expected != houses {
            println!("ERROR");
            println!("{}x2 + {}x + {}; x={}", a, b, c, x);
            println!("expected: {}, got {}", expected, houses);
            println!("problem {}", _i);
            panic!("values don't match");
        }

        output += &*houses.to_string();
        output += "\n";
    }

    output
}

fn solve(content: String, problems: usize, padding: usize) -> String {
    let content: Vec<&str> = content.lines().collect();
    let mut line_ptr: usize = padding;

    let mut output: String = String::new();

    for _i in 0..problems {
        let values: Vec<usize> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 1;

        let D = values[0];
        let K = values[1];
        let C = values[2];
        let L = values[3];

        /*let mut stored = 0;
        let mut production = 0;*/
        let mut houses = 0;

        /*let a = -K;
        let b = (-K + 2*K*D + 2*C*L);
        let c = -2*C*L*D;

        let d: f64 = (b*b - 4*a*c) as f64;

        let x1: f64 = (-b as f64 + d.sqrt()) as f64 / (2*a) as f64;
        let x2: f64 = (2*D - 1) as f64 / 2.0;

        let mut x = x1;

        if x2 < x1 {
            x = x2;
        }

        println!("{}", x);

        let houses = (((K as f64)*(x*x - x) / 2.0) +
            (K as f64)*x*((D as f64) - x))
            / (C as f64);*/

        for i in 1..D {
            let D_left = D - i;

            let a = (((K * (i * i - i) / 2) + K * i * D_left) as f64 / (C) as f64).floor() as usize;
            let b = L * D_left;

            let min = a.min(b);

            if min < houses {
                //println!("old {}", i);
                break;
            }

            houses = min;

            /*stored += production;
            let D_left = D - i;

            if D_left == 1 {
                houses = (((stored + production * (D_left - 1)) as f64 / (C) as f64)
                    .floor() as usize)
                    .min((L * D_left));
                break;
            }

            let aproduced = stored + production * (D_left - 1);

            // case 1: exploring
            let ehouses = (((aproduced + K * (D_left - 2)) as f64 / C as f64)
                .floor() as usize)
                .min((L * (D_left - 1)));

            // case 2: building
            let bhouses = ((aproduced as f64 / C as f64)
                .floor() as usize)
                .min((L * D_left));

            if bhouses > ehouses {
                houses = bhouses;
                break;
            }

            production += K;*/
        }

        output += &*houses.to_string();
        output += "\n";
    }

    output
}

fn simulate(D: i128, K: i128, C: i128, L: i128) -> i128 {
    let mut houses = 0;

    for i in 1..D {
        let D_left = D - i;

        let a = (((K * (i * i - i) / 2) + K * i * D_left) as f64 / (C) as f64).floor() as i128;
        let b = L * D_left;

        let min = a.min(b);

        if min < houses {
            //println!("old {}", i);
            break;
        }

        houses = min;
    }
    houses
}