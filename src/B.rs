use std::fs;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("B.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let mut bookshelf_raw = content.get(line_ptr + 1).unwrap().to_string();
        line_ptr += 2;

        //let n_books = bookshelf_raw.replace("_", "").len();

        /*let mut books: Vec<&str> = bookshelf_raw.split("").filter(|e| !e.is_empty()).collect();*/

        //let mut moves = 0;

        let moves = bookshelf_raw.trim_start_matches("_").trim_end_matches("_").replace("K", "").len();

        output += &*moves.to_string();
        output += "\n";

        /*for i in 0..books.len() {
            if books.join("").contains(&*"K".repeat(n_books)) {
                output += &*moves.to_string();
                output += "\n";
                break;
            }

            //println!("{} {:?}", i, books);
            if books.get(i).unwrap().to_string() == "_" {
                continue;
            }

            let j = books[i..].iter().position(|e| e.to_string() == "_").unwrap() + i;

            books[i] = "_";
            books[j] = "K";

            moves += 1;
        }*/

        //println!("Hotovo {} z {} problémů", _i, problems)
    }

    fs::write("B-out.txt", output.trim()).unwrap();
}