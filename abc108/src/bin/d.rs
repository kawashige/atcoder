use proconio::input;

fn main() {
    input! {
        l: usize,
    }

    if l < 5 {
        println!("2 {}", l);
        for i in 0..l {
            println!("1 2 {}", i);
        }
        return;
    }

    let l = l - 1;
    let b = format!("{:b}", l).chars().collect::<Vec<char>>();
    let v = b.len();
    let mut e = Vec::new();

    for i in 0..4 {
        e.push(format!("{} {} {}", v - 1, v, i));
    }

    for i in 0..(v - 3) {
        e.push(format!("{} {} {}", v - 2 - i, v - 1 - i, 0));
        e.push(format!(
            "{} {} {}",
            v - 2 - i,
            v - 1 - i,
            2_usize.pow(2 + i as u32)
        ));
    }

    let mut x = 0;
    for i in 0..(b.len() - 2) {
        if b[i] == '1' {
            e.push(format!("{} {} {}", 1, i + 2, x));
            x += 2_usize.pow((b.len() - 1 - i) as u32);
        }
    }

    match (b[b.len() - 2], b[b.len() - 1]) {
        ('0', '0') => e.push(format!("{} {} {}", 1, v, x)),
        ('0', '1') => {
            e.push(format!("{} {} {}", 1, v, x));
            e.push(format!("{} {} {}", 1, v, x + 1));
        }
        ('1', '0') => {
            e.push(format!("{} {} {}", 1, v, x));
            e.push(format!("{} {} {}", 1, v, x + 1));
            e.push(format!("{} {} {}", 1, v, x + 2));
        }
        _ => e.push(format!("{} {} {}", 1, v - 1, x)),
    }

    println!("{} {}", v, e.len());
    for edge in e {
        println!("{}", edge);
    }
}
