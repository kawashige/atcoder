use proconio::input;

fn main() {
    input! {
        b: [usize; 10],
        n: usize,
        mut a: [usize; n]
    }

    let mut map = [0; 10];
    for i in 0..10 {
        map[b[i]] = i;
    }

    a.sort_unstable_by_key(|x| {
        x.to_string()
            .chars()
            .fold(0, |acc, c| acc * 10 + map[c.to_digit(10).unwrap() as usize])
    });

    for x in a {
        println!("{}", x);
    }
}
