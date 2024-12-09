use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("D.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let values: Vec<i32> = content.get(line_ptr + 1).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 2;

        let mut profit_on = 0;
        let mut profit_off = 0;
        let mut profit_twice_off = 0;

        for a in values {
            let mprofit_on = a + profit_off.max(profit_twice_off);
            let mprofit_off = profit_on.max(profit_off);
            let mprofit_twice_off = profit_off.max(profit_twice_off);

            profit_on = mprofit_on;
            profit_off = mprofit_off;
            profit_twice_off = mprofit_twice_off;
        }

        /*values.insert(0, 0);
        values.push(0);

        let mut clone: Vec<(i32, usize)> = vec![];
        let mut map: HashMap<usize, usize> = HashMap::new();

        for i in 0..values.len() {
            if values[i] != 0 {
                clone.push((/*2 * */values[i]/* - (values[i + 1] + values[i - 1])*/, i));
                map.insert(i, clone.len() - 1);
            }

            /*if i % 1000 == 0 {
                println!("{}", i);
            }*/
        }

        clone.sort();

        if clone.len() > 3 {
            let mut test = clone.clone();

            test.pop().unwrap();
            test.pop().unwrap();

            if test.iter().map(|e| e.0).sum::<i32>() == 0 {
                profit = ((test.len() / 2) as f64).ceil() as i32 * values.get(1).unwrap();
                /*println!("quick");*/
                output += &*profit.to_string();
                output += "\n";
                continue;
            }
        }

        let mut clone = VecDeque::from(clone);

        for i in 0..clone.len() {
            let max = clone[i];
            let pos = max.1;

            if max.0 == i32::MIN {
                continue;
            }

            /*if values.get(pos - 1).unwrap_or(&-1) < &0 || values.get(pos + 1).unwrap_or(&-1) < &0 {
                values[pos] = 0;
                continue;
            }*/

            /*let mut before = *values.get_mut(pos - 1).unwrap_or(&mut 0);
            let mut after = *values.get_mut(pos + 1).unwrap_or(&mut 0);
*/
            if map.contains_key(&(pos - 1)) {
                clone[*map.get(&(pos - 1)).unwrap()] = (i32::MIN, 0);
            }
            //before = -1;
            if map.contains_key(&(pos + 1)) {
                clone[*map.get(&(pos + 1)).unwrap()] = (i32::MIN, 0);
            }
            //after = -1;

            profit += values[pos];
            //values[pos] = -1;

            /*if clone.len() % 100000 < 5 {
                println!("Zbývá {}", clone.len());
            }*/
        }*/

        println!("|");
        output += &*profit_on.max(profit_off).max(profit_twice_off).to_string();
        output += "\n";
    }

    fs::write("D-out.txt", output.trim()).unwrap();
    println!("Hotovo za {:?}", start.elapsed());
}