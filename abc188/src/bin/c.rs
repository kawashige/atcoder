use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; 2_usize.pow(n as u32)]
    }

    let i = (0..(a.len() / 2)).max_by_key(|i| a[*i]).unwrap();
    let j = ((a.len() / 2)..a.len()).max_by_key(|i| a[*i]).unwrap();

    if a[i] < a[j] {
        println!("{}", i + 1);
    } else {
        println!("{}", j + 1);
    }
}
