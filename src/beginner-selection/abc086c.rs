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

#[derive(Debug, Clone, Copy)]
struct Checkpoint {
    t: i32,
    x: i32,
    y: i32,
}

fn main() {
    let checkpoint_num = parse_line_to_single::<usize>();
    let mut checkpoints = vec![];

    checkpoints.push(Checkpoint { t: 0, x: 0, y: 0 });

    for _ in 0..checkpoint_num {
        parse_line![t: i32, x: i32, y: i32];
        let c = Checkpoint { t: t, x: x, y: y };
        checkpoints.push(c);
    }

    for w in checkpoints.windows(2) {
        let current = w[0];
        let next = w[1];

        let duration = next.t - current.t;

        let diff_x = (next.x - current.x).abs();
        let diff_y = (next.y - current.y).abs();
        let diff = diff_x + diff_y;

        let is_travelable = duration >= diff && (duration - diff) % 2 == 0;

        if !is_travelable {
            println!("No",);
            return;
        }
    }

    println!("Yes",);
}
