use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut odd: u64 = 1;
    let mut total: u64 = 1;

    for x in a {
        if x % 2 == 0 {
            odd *= 2;
        }
        total *= 3;
    }

    println!("{}", total - odd);
}
