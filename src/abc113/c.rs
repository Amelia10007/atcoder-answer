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

#[derive(Debug, Clone, Copy, PartialEq)]
struct City {
    id: usize,
    born_year: usize,
}

fn main() {
    parse_line![prefecture_count: usize, city_count: usize];

    let mut cities_on_prefectures = vec![vec![]; prefecture_count];

    for i in 1..city_count + 1 {
        parse_line![belonging_prefecture: usize, born_year: usize];

        let city = City {
            id: i,
            born_year: born_year,
        };

        let prefecture_index = belonging_prefecture - 1;

        cities_on_prefectures[prefecture_index].push(city);
    }

    let mut idstrings = vec![String::new(); city_count];

    for i in 1..prefecture_count + 1 {
        let prefecture_index = i - 1;
        let cities = &mut cities_on_prefectures[prefecture_index];
        cities.sort_by_key(|c| c.born_year);

        let id_left = to_id_string(i);
        for (index, city) in cities.into_iter().enumerate() {
            let id_right = to_id_string(index + 1);
            let idstring = format!("{}{}", id_left, id_right);

            let city_index = city.id - 1;
            idstrings[city_index] = idstring;
        }
    }

    for idstring in idstrings.into_iter() {
        println!("{}", idstring);
    }
}

fn to_id_string(x: usize) -> String {
    let right = x.to_string();
    let zero_count = 6 - right.len();

    let mut s = String::new();

    for _ in 0..zero_count {
        s.push('0');
    }
    s.push_str(&right);

    s
}
