use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut min = 0;
    let mut seen = vec![false; 200001];

    for x in p {
        seen[x] = true;
        while seen[min] {
            min += 1;
        }
        println!("{}", min);
    }
}
