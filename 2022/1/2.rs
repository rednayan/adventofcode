use std::fs;

fn main() {
    let x = fs::read_to_string("pz2.txt").unwrap();
    // let x = "1100\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000 ";
    let y: Vec<&str> = x.split("\n\n").collect();
    let mut sum: Vec<u32> = y
        .into_iter()
        .map(|yy| {
            yy.trim()
                .split("\n")
                .into_iter()
                .map(|c| c.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    sum.sort_by(|a, b| b.cmp(a));
    let mut top_three: u32 = 0;
    for i in 0..3 {
        top_three += sum[i];
    }
    println!("{top_three}");
}
