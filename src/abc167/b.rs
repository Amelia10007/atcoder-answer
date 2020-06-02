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
    parse_line![a: i32, b: i32, c: i32, k: i32];

    let max = if k < a {
        k
    } else if k < a + b {
        a
    } else if k < a + b + c {
        a - (k - (a + b))
    } else {
        a - c
    };

    println!("{}", max);
}
