use proconio::input;

fn main() {
    input! {
        n: usize,
        md: [String; n]
    }

    let mut year = vec![false; 366];
    let days = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut offset = vec![0; 12];
    for i in 1..12 {
        offset[i] += offset[i - 1] + days[i - 1];
    }

    for i in 0..year.len() {
        if i % 7 == 6 || i % 7 == 0 {
            year[i] = true;
        }
    }

    let mut holidays = md
        .into_iter()
        .map(|s| {
            let mut sp = s.split("/");
            let m = sp.next().unwrap().parse::<usize>().unwrap();
            let d = sp.next().unwrap().parse::<usize>().unwrap();
            (m, d)
        })
        .collect::<Vec<_>>();
    holidays.sort_unstable();

    for (m, d) in holidays {
        let mut i = offset[m - 1] + d - 1;
        while i < year.len() && year[i] {
            i += 1;
        }

        if i < year.len() {
            year[i] = true;
        }
    }

    let mut r = 0;
    let mut tmp = 0;
    for i in 0..year.len() {
        if year[i] {
            tmp += 1;
            r = r.max(tmp);
        } else {
            tmp = 0;
        }
    }

    println!("{}", r);
}
