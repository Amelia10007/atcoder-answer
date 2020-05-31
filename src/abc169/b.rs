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
    parse_line![_n: usize];

    let mut nums = parse_line_to_multiple::<u128>();

    nums.sort();

    if nums[0] == 0 {
        println!("0",);
        return;
    }

    let mut product = 1;

    for x in nums.into_iter() {
        product *= x;

        if product > 1000000000000000000 {
            println!("-1",);
            return;
        }
    }

    println!("{}", product);
}
