use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
#[allow(deprecated)]
fn parse_line_to_single<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    T::from_str(buffer.trim_right()).unwrap()
}

struct Fact {
    base: u64,
    power: u64,
}

fn factorization(n: u64) -> Vec<Fact> {
    let mut facts = vec![];
    let mut remaining = n;
    let max_candidate = (n as f64).sqrt() as u64;

    for candidate in 2..max_candidate + 1 {
        if remaining % candidate == 0 {
            let mut count = 0;
            while remaining % candidate == 0 {
                count += 1;
                remaining /= candidate;
            }
            facts.push(Fact {
                base: candidate,
                power: count,
            });
        }
    }

    if remaining != 1 {
        facts.push(Fact {
            base: remaining,
            power: 1,
        });
    }

    facts
}

fn main() {
    let n = parse_line_to_single::<u64>();

    if n == 1 {
        println!("0",);
        return;
    }

    let facts = factorization(n);

    let mut operation_count = 0;

    for Fact { base: _, mut power } in facts.into_iter() {
        let mut temp_operation_count = 0;
        let mut current_power = 1;

        while power >= current_power {
            power -= current_power;
            current_power += 1;
            temp_operation_count += 1;
        }

        operation_count += temp_operation_count;
    }

    println!("{}", operation_count);
}
