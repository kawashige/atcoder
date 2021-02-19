use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pos: [(i32, i32); n]
    }

    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if pos[j].0 != pos[i].0 {
                let d = (pos[j].1 - pos[i].1) as f32 / (pos[j].0 - pos[i].0) as f32;
                if -1.0 <= d && d <= 1.0 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
