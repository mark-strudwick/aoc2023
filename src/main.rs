mod days;

fn main() {
    println!("Hello, world!");
    let day1a = days::day1::part1();
    println!("Day1: a: {:?}", day1a.unwrap());

    let day1b = days::day1::part2();
    println!("Day1: b: {:?}", day1b.unwrap());
}
