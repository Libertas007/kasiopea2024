use std::fs;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("A.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let values: Vec<f64> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 1;

        let count = *values.get(0).unwrap();
        let rezacka_time = *values.get(1).unwrap();
        let nuzky_time = *values.get(2).unwrap();
        let nuzky_effectivity = *values.get(3).unwrap();

        let rezacka_total = (rezacka_time * count) as i32;

        let nuzky_total = ((count / nuzky_effectivity).ceil() * nuzky_time) as i32;

        let i = (count / nuzky_effectivity).floor();
        let nuzky_total_time = i * nuzky_time;

        let j = (count % nuzky_effectivity);
        let rezacka_total_time = j * rezacka_time;

        let combined_total = (nuzky_total_time + rezacka_total_time) as i32;


        output += &*vec![rezacka_total, nuzky_total, combined_total].iter().min().unwrap().to_string();
        output += "\n";
    }

    fs::write("A-out.txt", output.trim()).unwrap();
}