#[allow(dead_code)]
#[allow(deprecated)]
fn read_line_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_right().to_owned()
}

fn main() {
    let s = read_line_from_stdin();
    let cs = s.chars().collect::<Vec<_>>();

    if verify(&cs, 0) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn verify(goal_string: &[char], current_index: usize) -> bool {
    if current_index >= goal_string.len() {
        return true;
    }
    if current_index + 5 > goal_string.len() {
        false
    } else if &goal_string[current_index..current_index + 5] == &['d', 'r', 'e', 'a', 'm'] {
        if goal_string.len() == current_index + 5 {
            return true;
        }
        if goal_string.len() >= current_index + 7
            && &goal_string[current_index + 5..current_index + 7] == &['e', 'r']
            && verify(goal_string, current_index + 7)
        {
            return true;
        }
        verify(goal_string, current_index + 5)
    } else if &goal_string[current_index..current_index + 5] == &['e', 'r', 'a', 's', 'e'] {
        if goal_string.len() == current_index + 6 {
            return true;
        }
        if goal_string.len() >= current_index + 6
            && &goal_string[current_index + 5..current_index + 6] == &['r']
            && verify(goal_string, current_index + 6)
        {
            return true;
        }
        verify(goal_string, current_index + 5)
    } else {
        false
    }
}
