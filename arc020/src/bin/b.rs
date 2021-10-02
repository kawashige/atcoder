use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        a: [usize; n]
    }

    let mut min = std::usize::MAX;
    for i in 1..11 {
        let cost = (0..n)
            .step_by(2)
            .map(|k| if a[k] == i { 0 } else { c })
            .sum::<usize>();
        for j in 1..11 {
            if i == j {
                continue;
            }
            let cost2 = cost
                + (1..n)
                    .step_by(2)
                    .map(|k| if a[k] == j { 0 } else { c })
                    .sum::<usize>();

            min = min.min(cost2);
        }
    }

    println!("{}", min);
}
