use std::fs;

fn main() {
    let x = fs::read_to_string("pz1.txt").unwrap();
    // let x = "1100\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000 ";
    let y: Vec<&str> = x.split("\n\n").collect();
    let sum: u32 = y
        .into_iter()
        .map(|yy| {
            yy.trim()
                .split("\n")
                .into_iter()
                .map(|c| c.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();
    println!("{sum}");
}
