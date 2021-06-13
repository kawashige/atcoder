use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut four = 0;
    let mut two = 0;
    for x in a {
        if x % 4 == 0 {
            four += 1;
        } else if x % 2 == 0 {
            two += 1;
        }
    }

    if n - (four + if two > 1 { two - 1 } else { 0 }) > four + 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
