use std::str::FromStr;

#[allow(unused_macros)]
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
    parse_line![a: u64, s: String];

    let mut split = s.split('.');
    let b100 = {
        let integer = u64::from_str(split.next().unwrap()).unwrap();
        let under_dot = u64::from_str(split.next().unwrap()).unwrap();

        integer * 100 + under_dot
    };

    println!("{}", a * b100 / 100);
}
