use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut remains: Vec<Vec<usize>> = vec![vec![]; 200];

    for i in 0..n {
        let r = a[i] % 200;
        let mut v: Vec<Vec<usize>> = vec![vec![]; 200];
        v[r] = vec![i];

        for j in 1..200 {
            if !remains[j].is_empty() {
                let mut tmp = remains[j].clone();
                tmp.push(i);
                v[(j + r) % 200] = tmp;
            }
        }

        for j in 0..200 {
            if !remains[j].is_empty() && !v[j].is_empty() {
                println!("Yes");
                println!(
                    "{} {}",
                    remains[j].len(),
                    remains[j]
                        .iter()
                        .map(|x| (x + 1).to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                println!(
                    "{} {}",
                    v[j].len(),
                    v[j].iter()
                        .map(|x| (x + 1).to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                return;
            } else if remains[j].is_empty() && !v[j].is_empty() {
                remains[j] = v[j].clone();
            }
        }
    }

    println!("No");
}
