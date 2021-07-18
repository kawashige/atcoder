use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: [usize; n]
    }

    let mut count = vec![0; 3];
    for x in p {
        if x <= a {
            count[0] += 1;
        } else if x <= b {
            count[1] += 1;
        } else {
            count[2] += 1;
        }
    }

    println!("{}", count.iter().min().unwrap());
}
