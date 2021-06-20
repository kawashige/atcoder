use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    if ([4, 6, 9, 11].contains(&x) && [4, 6, 9, 11].contains(&y))
        || ([1, 3, 5, 7, 8, 10, 12].contains(&x) && [1, 3, 5, 7, 8, 10, 12].contains(&y))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
