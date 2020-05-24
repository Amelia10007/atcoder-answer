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
fn parse_line_to_single<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    T::from_str(buffer.trim_right()).unwrap()
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

macro_rules! parse_line {
    [$( $x:tt : $t:ty ),+] => {
        //declare variables
        $( let $x: $t; )+
        {
            // read
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            #[allow(deprecated)]
            let mut splits = buf.trim_right().split_whitespace();
            // assign each variable
            $(
                $x = splits.next().and_then(|s| <$t>::from_str(s).ok()).unwrap();
            )+
            // all strs should be used for assignment
            assert!(splits.next().is_none());
        }
    };
}

fn main() {
    use std::cmp::min;

    parse_line![a: i32];
    parse_line![b: i32];
    parse_line![c: i32];
    parse_line![sum: i32];

    let mut pattern_count = 0;

    let max_count500 = min(a, sum / 500);

    for count500 in 0..max_count500 + 1 {
        let remaining = sum - 500 * count500;
        if remaining < 0 {
            break;
        }

        let max_count100 = min(b, remaining / 100);

        for count100 in 0..max_count100 + 1 {
            let remaining = remaining - count100 * 100;
            if remaining < 0 {
                break;
            }

            let count50 = remaining / 50;

            if count50 <= c {
                pattern_count += 1;
            }
        }
    }

    println!("{}", pattern_count);
}
