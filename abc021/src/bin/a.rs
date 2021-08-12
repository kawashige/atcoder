use proconio::input;

fn main() {
    input! {
        mut n: usize
    }

    let mut r = vec![];
    while n >= 8 {
        r.push(8);
        n -= 8;
    }
    while n >= 4 {
        r.push(4);
        n -= 4;
    }
    while n >= 2 {
        r.push(2);
        n -= 2;
    }
    while n >= 1 {
        r.push(1);
        n -= 1;
    }

    println!("{}", r.len());
    for x in r {
        println!("{}", x);
    }
}
