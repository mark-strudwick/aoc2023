mod days;

fn main() {
    let day1a = days::day1::part1();
    println!("Day1: a: {:?}", day1a.unwrap());

    let day1b = days::day1::part2();
    println!("Day1: b: {:?}", day1b.unwrap());

    let day2a = days::day2::part1();
    println!("Day2: a: {:?}", day2a.unwrap());

    let day2b = days::day2::part2();
    println!("Day2: b: {:?}", day2b.unwrap());
}
