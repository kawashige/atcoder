use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        c: i32,
        s: [String; n]
    }

    if a == 0 && b == 0 && c == 0 {
        println!("No");
        return;
    }

    let mut count = vec![a, b, c];
    let mut result = Vec::new();
    for i in 0..n {
        let (x, y) = match s[i].as_str() {
            "AB" => (0, 1),
            "AC" => (0, 2),
            "BC" => (1, 2),
            _ => unreachable!(),
        };

        if count[x] < count[y] {
            count[x] += 1;
            count[y] -= 1;
            result.push(x);
        } else if count[x] > count[y] {
            count[y] += 1;
            count[x] -= 1;
            result.push(y);
        } else {
            if i < n - 1 {
                let (x2, y2) = match s[i + 1].as_str() {
                    "AB" => (0, 1),
                    "AC" => (0, 2),
                    "BC" => (1, 2),
                    _ => unreachable!(),
                };
                if x == x2 || x == y2 {
                    count[x] += 1;
                    count[y] -= 1;
                    result.push(x);
                } else {
                    count[y] += 1;
                    count[x] -= 1;
                    result.push(y);
                }
            } else {
                count[x] += 1;
                count[y] -= 1;
                result.push(x);
            }
        }
        if count[x] < 0 || count[y] < 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for r in result {
        println!(
            "{}",
            match r {
                0 => 'A',
                1 => 'B',
                _ => 'C',
            }
        );
    }
}
