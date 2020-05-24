use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
#[allow(deprecated)]
fn read_line_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_right().to_owned()
}

#[allow(dead_code)]
#[allow(deprecated)]
fn parse_line_to_multiple<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
        .trim_right()
        .split_whitespace()
        .flat_map(|s| T::from_str(s))
        .collect()
}

fn main() {
    let _ = read_line_from_stdin();
    let mut values = parse_line_to_multiple::<i32>();
    let mut operation_count = 0;

    while values.iter().all(|x| *x % 2 == 0) {
        for x in values.iter_mut() {
            *x /= 2;
        }

        operation_count += 1;
    }

    println!("{}", operation_count);
}
