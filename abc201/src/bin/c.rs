use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let maru = (0..10).filter(|i| s[*i] == 'o').collect::<Vec<usize>>();
    let hatena = (0..10).filter(|i| s[*i] == '?').collect::<Vec<usize>>();

    let mut count = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                for l in 0..10 {
                    if maru.iter().all(|x| [i, j, k, l].contains(x))
                        && [i, j, k, l]
                            .iter()
                            .all(|x| maru.contains(x) || hatena.contains(x))
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
