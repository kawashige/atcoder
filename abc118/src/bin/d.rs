use proconio::input;

fn dfs(
    i: usize,
    nums: &Vec<(usize, usize)>,
    remains: usize,
    l: usize,
    max: &mut usize,
    tmp: &mut Vec<(usize, usize)>,
    result: &mut Vec<(usize, usize)>,
) {
    if remains == 0 {
        if l > *max {
            *max = l;
            *result = tmp.clone();
        }
    }

    if i == nums.len() || (l + remains / nums[i..].iter().map(|(x, _)| *x).min().unwrap()) < *max {
        return;
    }

    for c in (0..=(remains / nums[i].0)).rev() {
        tmp.push((nums[i].1, c));

        dfs(
            i + 1,
            nums,
            remains - c * nums[i].0,
            l + c,
            max,
            tmp,
            result,
        );

        tmp.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    let count = vec![0, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut a = a
        .into_iter()
        .map(|i| (count[i], i))
        .collect::<Vec<(usize, usize)>>();
    a.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut nums = vec![a[0]];
    for i in 1..m {
        if a[i - 1].0 != a[i].0 {
            nums.push(a[i]);
        }
    }

    nums.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut r = Vec::new();
    dfs(0, &nums, n, 0, &mut 0, &mut Vec::new(), &mut r);
    r.sort_unstable_by(|a, b| b.cmp(&a));

    println!(
        "{}",
        r.into_iter()
            .map(|(d, c)| if c > 0 {
                std::iter::repeat((b'0' + d as u8) as char)
                    .take(c)
                    .collect::<String>()
            } else {
                "".to_string()
            })
            .collect::<String>()
    );
}
