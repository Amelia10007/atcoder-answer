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
    parse_line![
        hour_hand_len: f64,
        minute_hand_len: f64,
        hour: f64,
        minute: f64
    ];

    let hour_hand_rotation = hour / 12.0 + minute / (12.0 * 60.0);
    let minute_hand_rotation = minute / 60.0;
    let rotation_diff = {
        let mut temp = hour_hand_rotation - minute_hand_rotation;

        while temp > 1.0 {
            temp -= 1.0;
        }
        while temp < 0.0 {
            temp += 1.0;
        }

        temp
    };

    let diff_radian = rotation_diff * std::f64::consts::PI * 2.0;

    let distance = (hour_hand_len * hour_hand_len + minute_hand_len * minute_hand_len
        - 2.0 * hour_hand_len * minute_hand_len * diff_radian.cos())
    .sqrt();

    println!("{}", distance);
}
