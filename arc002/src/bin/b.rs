use proconio::input;

fn is_leap_year(y: usize) -> bool {
    y % 400 == 0 || (y % 4 == 0 && y % 100 != 0)
}

fn main() {
    input! {
        date: String
    }

    let (mut y, mut m, mut d) = {
        let mut sp = date.split('/');
        (
            sp.next().unwrap().parse::<usize>().unwrap(),
            sp.next().unwrap().parse::<usize>().unwrap(),
            sp.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    while y % (m * d) != 0 {
        d += 1;
        if (m == 2 && (is_leap_year(y) && d == 30 || !is_leap_year(y) && d == 29))
            || ([4, 6, 9, 11].contains(&m) && d == 31)
            || d == 32
        {
            d = 1;
            m += 1;
        }

        if m == 13 {
            y += 1;
            m = 1;
        }
    }

    println!("{}/{:02}/{:02}", y, m, d)
}
