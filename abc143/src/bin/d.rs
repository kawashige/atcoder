use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [usize; n]
    }

    l.sort_unstable();

    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let min = l[j] - l[i];
            let max = l[j] + l[i];
            count += ((j + 1)..n)
                .skip_while(|m| l[*m] <= min)
                .take_while(|m| l[*m] < max)
                .count()
        }
    }

    println!("{}", count);
}
