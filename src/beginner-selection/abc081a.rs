#[allow(dead_code)]
#[allow(deprecated)]
fn read_line_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_right().to_owned()
}

fn main() {
    println!(
        "{}",
        read_line_from_stdin().chars().filter(|c| *c == '1').count()
    );
}
