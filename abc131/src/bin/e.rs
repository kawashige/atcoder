use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    if k > (n - 1) * (n - 2) / 2 {
        println!("-1");
        return;
    }

    let mut m = n - 1;

    let mut edges = Vec::new();
    for i in 2..=n {
        edges.push((1, i))
    }

    let mut remains = (n - 1) * (n - 2) / 2 - k;
    for i in 2..=n {
        if remains == 0 {
            break;
        }
        for j in (i + 1)..=n {
            edges.push((i, j));
            remains -= 1;
            m += 1;
            if remains == 0 {
                break;
            }
        }
    }

    println!("{}", m);
    for (f, t) in edges {
        println!("{} {}", f, t);
    }
}
