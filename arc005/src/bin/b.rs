use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: usize,
        y: usize,
        w: String,
        c: [Chars; 9]
    }
    print!("{}", c[y - 1][x - 1]);

    let mut p = (y as i32 - 1, x as i32 - 1);
    let mut d = match w.as_str() {
        "R" => (0, 1),
        "L" => (0, -1),
        "U" => (-1, 0),
        "D" => (1, 0),
        "RU" => (-1, 1),
        "RD" => (1, 1),
        "LU" => (-1, -1),
        "LD" => (1, -1),
        _ => unreachable!(),
    };

    for _ in 0..3 {
        let new_p = (p.0 + d.0, p.1 + d.1);
        if new_p.0 < 0 || 8 < new_p.0 {
            d.0 *= -1
        }
        if new_p.1 < 0 || 8 < new_p.1 {
            d.1 *= -1
        }

        p = (p.0 + d.0, p.1 + d.1);
        print!("{}", c[p.0 as usize][p.1 as usize]);
    }
    print!("\n");
}
