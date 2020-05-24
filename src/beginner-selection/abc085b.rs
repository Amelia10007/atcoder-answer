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

macro_rules! parse_line {
    [$( $x:tt : $t:ty ),+] => {
        //declare variables
        $( let $x: $t; )+
        {
            use std::str::FromStr;
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
    parse_line![mochi_num: usize];

    let mut diameters = vec![];

    for _ in 0..mochi_num {
        diameters.push(parse_line_to_single::<u32>());
    }

    diameters.sort();
    diameters.dedup();

    println!("{}", diameters.len());
}
