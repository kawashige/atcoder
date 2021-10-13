use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [usize; n]
    }

    let mut v = Vec::with_capacity(n);

    for x in w {
        if let Some(i) = (0..v.len()).find(|i| x <= v[*i]) {
            v[i] = x;
        } else {
            v.push(x);
        }
    }

    println!("{}", v.len());
}
