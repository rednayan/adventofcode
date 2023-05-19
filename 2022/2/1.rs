use std::fs;

fn main() {
    let x = fs::read_to_string("pz1.txt").unwrap();
    // let x = "A Y\nB X\nC Z";
    let y: Vec<_> = x
        .trim()
        .split("\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .collect();

    let mut f_score: u32 = 0;

    for y1 in &y {
        if y1[1] == "X" {
            f_score += 1;
            if y1[0] == "A" {
                f_score += 3;
            } else if y1[0] == "B" {
                f_score += 0;
            } else if y1[0] == "C" {
                f_score += 6;
            }
        }
        if y1[1] == "Y" {
            f_score += 2;
            if y1[0] == "A" {
                f_score += 6;
            } else if y1[0] == "B" {
                f_score += 3;
            } else if y1[0] == "C" {
                f_score += 0;
            }
        }
        if y1[1] == "Z" {
            f_score += 3;
            if y1[0] == "A" {
                f_score += 0;
            } else if y1[0] == "B" {
                f_score += 6;
            } else if y1[0] == "C" {
                f_score += 3;
            }
        }
    }
    println!("{f_score}");
}
