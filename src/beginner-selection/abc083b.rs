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
    parse_line![max_num: u32, min_sum: u32, max_sum: u32];

    let mut sum_of_match = 0;

    for n in 1..max_num + 1 {
        let digit_sum = n
            .to_string()
            .chars()
            .flat_map(|c| u32::from_str(&format!("{}", c)))
            .sum::<u32>();

        if digit_sum >= min_sum && digit_sum <= max_sum {
            sum_of_match += n;
        }
    }

    println!("{}", sum_of_match);
}
