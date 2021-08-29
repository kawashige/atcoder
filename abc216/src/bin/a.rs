use proconio::input;

fn main() {
    input! {
        s: String
    }

    let v = s
        .split(".")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    if v[1] <= 2 {
        println!("{}-", v[0])
    } else if 3 <= v[1] && v[1] <= 6 {
        println!("{}", v[0])
    } else {
        println!("{}+", v[0])
    }
}
