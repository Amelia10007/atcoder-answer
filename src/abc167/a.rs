fn read_line_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_right().to_owned()
}

fn main() {
    let s = read_line_from_stdin();
    let t = read_line_from_stdin();

    if &s[..] == &t[..t.len() - 1] {
        println!("Yes",);
    } else {
        println!("No",);
    }
}
