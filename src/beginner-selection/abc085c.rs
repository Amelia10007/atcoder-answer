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
    use std::cmp::min;

    parse_line![bill_count: i32, amount: i32];

    let max_count10000 = amount / 10000;

    for count10000 in 0..max_count10000 + 1 {
        let remaining = amount - count10000 * 10000;
        if remaining < 0 {
            break;
        }

        let remaining_bill_count = bill_count - count10000;
        if remaining_bill_count < 0 {
            break;
        }

        let max_count5000 = min(remaining_bill_count, remaining / 5000);

        for count5000 in 0..max_count5000 + 1 {
            let count1000 = remaining_bill_count - count5000;
            if count1000 < 0 {
                break;
            }

            let remaining = remaining - count5000 * 5000;
            if remaining < 0 {
                break;
            }

            let count_cond = count10000 + count5000 + count1000 == bill_count;
            let amount_cond = remaining == count1000 * 1000;

            if count_cond && amount_cond {
                println!("{} {} {}", count10000, count5000, count1000);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
