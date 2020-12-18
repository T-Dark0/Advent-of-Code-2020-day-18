use std::{fs, time::Instant};

fn main() {
    let beginning = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();
    let file_read = Instant::now();

    part1(&input);
    let part1_done = Instant::now();

    part2(&input);
    let part2_done = Instant::now();

    let file_io = file_read.duration_since(beginning);
    let part1 = part1_done.duration_since(file_read);
    let part2 = part2_done.duration_since(part1_done);

    println!(
        "File IO: {:?}\nPart 1: {:?}\nPart2: {:?}",
        file_io, part1, part2
    );
}

fn part1(input: &str) {
    let total_sum = input
        .lines()
        .map(|line| part1_parser::expression(line).unwrap())
        .sum::<u64>();
    println!("Part 1 total sum: {}", total_sum);
}

fn part2(input: &str) {
    let total_sum = input
        .lines()
        .map(|line| part2_parser::expression(line).unwrap())
        .sum::<u64>();
    println!("Part 2 total sum: {}", total_sum);
}

peg::parser! {
    grammar part1_parser() for str {
        pub rule expression() -> u64 = precedence!{
                x:(@) _ "+" _ y:@ { x + y }
                x:(@) _ "*" _ y:@ { x * y }
                "(" _ e:expression() _ ")" { e }
                n:number() { n }
        }

        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule _() = " "?
    }
}

peg::parser! {
    grammar part2_parser() for str {
        pub rule expression() -> u64 = precedence!{
                x:(@) _ "*" _ y:@ { x * y }
                --
                x:(@) _ "+" _ y:@ { x + y }
                --
                "(" _ e:expression() _ ")" { e }
                n:number() { n }
        }

        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule _() = " "?
    }
}
