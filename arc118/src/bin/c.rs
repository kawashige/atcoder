use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut r = vec![6, 10, 15];

    if n > 3 {
        for i in 16..10001 {
            if i % 6 == 0 || i % 10 == 0 || i % 15 == 0 {
                r.push(i);
                if r.len() == n {
                    break;
                }
            }
        }
    }

    println!(
        "{}",
        r.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
