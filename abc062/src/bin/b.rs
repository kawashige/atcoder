use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [String; h]
    }

    println!("{}", std::iter::repeat('#').take(w + 2).collect::<String>());
    for i in 0..h {
        println!("#{}#", a[i]);
    }
    println!("{}", std::iter::repeat('#').take(w + 2).collect::<String>());
}
