use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    let mut grid = vec![vec![]; 100];
    for i in 0..100 {
        grid[i] = vec![if i < 50 { '.' } else { '#' }; 100];
    }

    a -= 1;
    b -= 1;

    for i in (0..48).step_by(2) {
        if b == 0 {
            break;
        }
        for j in (1..100).step_by(2) {
            grid[i][j] = '#';
            b -= 1;
            if b == 0 {
                break;
            }
        }
    }
    for i in (51..100).step_by(2) {
        if a == 0 {
            break;
        }
        for j in (0..100).step_by(2) {
            grid[i][j] = '.';
            a -= 1;
            if a == 0 {
                break;
            }
        }
    }

    println!("100 100");
    for i in 0..100 {
        println!("{}", grid[i].iter().collect::<String>());
    }
}
