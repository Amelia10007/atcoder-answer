use std::fmt::Debug;
use std::str::FromStr;

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
    parse_line![_card_num: usize];
    let mut card_values = parse_line_to_multiple::<u32>();
    card_values.sort();

    let mut alice_score = 0;
    let mut bob_score = 0;

    let mut is_alice_turn = true;

    while let Some(value) = card_values.pop() {
        if is_alice_turn {
            alice_score += value;
        } else {
            bob_score += value;
        }
        is_alice_turn = !is_alice_turn;
    }

    println!("{}", alice_score - bob_score);
}
