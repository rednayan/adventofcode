use std::fs;
fn main() {
    let x = fs::read_to_string("pz1.txt").unwrap();
    // let x = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
    // .to_string();
    let mut v = Vec::new();
    let mut z = '1';

    let y: Vec<_> = x.trim().split("\n").collect::<Vec<_>>();
    y.iter().for_each(|x| {
        let (first, last) = x.split_at(x.len() / 2);
        first.chars().for_each(|ch| {
            last.chars().for_each(|ch2| {
                if ch == z {
                    return;
                } else if ch == ch2 {
                    v.push(ch);
                    z = ch;
                }
            });
        });
    });
    let mut sum = 0;
    v.iter().for_each(|&c| {
        if c >= 'a' {
            sum += c as u32 - 96;
        } else {
            sum += c as u32 - 38
        }
    });
    println!("{}", sum);
}
