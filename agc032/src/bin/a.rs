use proconio::input;

fn main() {
    input! {
        n: usize,
        mut b: [usize; n]
    }

    let mut r = Vec::with_capacity(n);

    while !b.is_empty() {
        let mut delete = false;
        for i in (0..b.len()).rev() {
            if b[i] == i + 1 {
                r.push(b[i]);
                b.remove(i);
                delete = true;
                break;
            }
        }

        if !delete {
            println!("-1");
            return;
        }
    }

    for x in r.into_iter().rev() {
        println!("{}", x);
    }
}
