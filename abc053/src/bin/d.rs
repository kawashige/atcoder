use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut count = vec![0; 100001];
    for x in a {
        count[x] += 1;
    }

    let mut c = 0;
    let mut r = 0;

    for x in count {
        if x == 0 {
            continue;
        }
        c += 1;
        r += x - 1;
    }

    if r % 2 == 1 {
        c -= 1;
    }

    println!("{}", c);
}
