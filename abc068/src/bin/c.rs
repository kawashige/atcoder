use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut list = vec![];
    let mut to_n = vec![false; n];
    for (a, b) in ab {
        if a == 1 {
            list.push(b - 1);
        } else if b == 1 {
            list.push(a - 1);
        }

        if a == n {
            to_n[b - 1] = true;
        } else if b == n {
            to_n[a - 1] = true;
        }
    }

    if list.into_iter().any(|x| to_n[x]) {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
