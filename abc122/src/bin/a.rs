use proconio::input;

fn main() {
    input! {
        b: char
    }

    let c = match b {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => unreachable!(),
    };

    println!("{}", c);
}
