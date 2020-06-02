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

use std::collections::HashSet;

fn main() {
    parse_line![_town_count: usize, desired_move_count: usize];

    let transporters = parse_line_to_multiple::<usize>();

    let mut current_town = 1;
    let mut move_histories = vec![];
    let mut visited_towns = HashSet::new();

    loop {
        if move_histories.len() == desired_move_count {
            println!("{}", current_town);
            break;
        }
        if visited_towns.contains(&current_town) {
            let remaining_movement = desired_move_count - move_histories.len();

            let first_loop_index = move_histories
                .iter()
                .enumerate()
                .find(|(_i, h)| **h == current_town)
                .map(|(i, _)| i)
                .unwrap();

            let loop_length = move_histories.len() - first_loop_index;
            let index_diff = remaining_movement % loop_length;
            let final_town = move_histories[first_loop_index + index_diff];
            println!("{}", final_town);
            break;
        }
        move_histories.push(current_town);
        visited_towns.insert(current_town);
        let town_index = current_town - 1;
        current_town = transporters[town_index];
    }
}
