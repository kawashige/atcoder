use proconio::input;

fn main() {
    input! {
        d: [usize; 7],
        j: [usize; 7]
    }

    println!("{}", (0..7).map(|i| d[i].max(j[i])).sum::<usize>());
}
