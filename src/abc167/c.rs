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
fn parse_line_to_single<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    T::from_str(buffer.trim_right()).unwrap()
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

#[derive(Debug, Clone, PartialEq)]
struct Book {
    price: usize,
    skills: Vec<usize>,
}

fn pow(base: usize, mut power: usize) -> usize {
    let mut value = base;
    while power > 1 {
        value *= base;
        power -= 1;
    }

    value
}

fn main() {
    parse_line![book_count: usize, algol_count: usize, desired_skill: usize];

    let books = {
        let mut books = vec![];
        for _ in 0..book_count {
            let line = parse_line_to_multiple::<usize>();
            books.push(Book {
                price: line[0],
                skills: line[1..].to_vec(),
            });
        }
        books
    };

    let mut lowest_price = None;

    for i in 0..pow(2, book_count) + 1 {
        let is_selecteds = (0..book_count).map(|b| i & (1 << b) != 0);

        let selected_books = books
            .iter()
            .zip(is_selecteds)
            .filter(|&(_, is_selected)| is_selected)
            .map(|(book, _)| book)
            .collect::<Vec<_>>();

        let mut obtained_skills = vec![0; algol_count];
        for book in selected_books.iter() {
            for (j, skill) in book.skills.iter().enumerate() {
                obtained_skills[j] += *skill;
            }
        }

        let is_achieved = obtained_skills.into_iter().all(|s| s >= desired_skill);

        if is_achieved {
            let price_sum = selected_books.iter().map(|b| b.price).sum::<usize>();
            match lowest_price {
                Some(p) if p > price_sum => {
                    lowest_price = Some(price_sum);
                }
                None => {
                    lowest_price = Some(price_sum);
                }
                _ => {}
            }
        }
    }

    match lowest_price {
        Some(p) => println!("{}", p),
        None => println!("-1",),
    }
}
