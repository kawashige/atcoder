use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
    }

    let num = format!("{}{}{}", r, g, b).parse::<usize>().unwrap();
    if num % 4 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
