use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: usize
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut q: i32 = 0;
    for c in s {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => q += 1,
        }
    }

    if t == 1 {
        println!("{}", x.abs() + y.abs() + q);
    } else {
        if x.abs() + y.abs() >= q {
            println!("{}", (x.abs() + y.abs() - q).abs());
        } else if (q - (x.abs() + y.abs())) % 2 == 0 {
            println!("0");
        } else {
            println!("1");
        }
    }
}
