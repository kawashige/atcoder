use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut max = 0;
    let mut max_i = 1;
    for i in 1..=n {
        let mut count = 0;
        let mut x = i;
        while x % 2 == 0 {
            x /= 2;
            count += 1;
        }
        if max < count {
            max_i = i;
            max = count;
        }
    }

    println!("{}", max_i);
}
