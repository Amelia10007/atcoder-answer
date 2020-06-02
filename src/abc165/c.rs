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
struct Query {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

fn main() {
    parse_line![n: usize, m: usize, q: usize];

    let queries = {
        let mut temp = vec![];
        for _ in 0..q {
            parse_line![a: usize, b: usize, c: usize, d: usize];
            temp.push(Query {
                a: a,
                b: b,
                c: c,
                d: d,
            });
        }
        temp
    };

    let mut max_score = 0;
    let mut nums = vec![1; n];

    loop {
        let score = queries
            .iter()
            .filter(|&&Query { a, b, c, .. }| nums[b - 1] - nums[a - 1] == c)
            .map(|&Query { d, .. }| d)
            .sum::<usize>();

        max_score = std::cmp::max(max_score, score);

        if next_nums(&mut nums, m).is_none() {
            break;
        }
    }

    println!("{} ", max_score);
}

fn next_nums(nums: &mut [usize], m: usize) -> Option<()> {
    let len = nums.len();
    // この位の値をカウントアップ
    nums[len - 1] += 1;
    if nums[len - 1] > m {
        // これ以上位上げができなければ終了
        if len - 1 == 0 {
            None
        } else {
            // 次の位をカウントアップ
            next_nums(&mut nums[..len - 1], m)?;
            // この数列は単調非減少だから
            // この位の値が次の位の値と同じになるようにする
            nums[len - 1] = nums[len - 2];
            Some(())
        }
    } else {
        Some(())
    }
}
