use std::fs;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("C.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let mut values: Vec<i32> = content.get(line_ptr + 1).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 2;

        values.sort();

        let clone = values.clone();

        values.dedup();

        let mut max = 0;
        for a in values {
            let count = clone.iter().filter(|e| e == &&a).count();
            if max < count {
                max = count;
            }
        }

        println!("|");
        output += &*max.to_string();
        output += "\n";
    }

    fs::write("C-out.txt", output.trim()).unwrap();
}