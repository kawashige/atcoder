use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String
    }

    let mut left = Vec::new();
    let mut right = Vec::new();

    let target = (s.len() - 1) / 2;

    while left.len() < target {
        left.push('a');
        right.push('c');

        left.push('c');
        right.push('a');

        left.push('b');
        right.push('b');
    }

    while left.len() > target {
        left.pop();
    }
    while right.len() > target {
        right.pop();
    }

    if s == left
        .into_iter()
        .rev()
        .chain(std::iter::once('b'))
        .chain(right.into_iter())
        .collect::<String>()
    {
        println!("{}", target);
    } else {
        println!("-1");
    }
}
