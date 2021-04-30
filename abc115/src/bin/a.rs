use proconio::input;

fn main() {
    input! {
        d: usize
    }

    let r = match d {
        25 => "Christmas",
        24 => "Christmas Eve",
        23 => "Christmas Eve Eve",
        22 => "Christmas Eve Eve Eve",
        _ => unreachable!(),
    };

    println!("{}", r);
}
