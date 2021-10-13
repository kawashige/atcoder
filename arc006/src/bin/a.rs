use proconio::input;

fn main() {
    input! {
        e: [usize; 6],
        b: usize,
        l: [usize; 6]
    }

    let c = l.iter().filter(|x| e.contains(*x)).count();
    let r = match c {
        6 => 1,
        5 => {
            if (0..6).any(|i| e[i] != l[i] && l[i] == b) {
                2
            } else {
                3
            }
        }
        4 => 4,
        3 => 5,
        _ => 0,
    };
    println!("{}", r);
}
