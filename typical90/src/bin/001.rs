use proconio::input;

fn check(arr: &Vec<usize>, k: usize, target: usize) -> bool {
    let mut k = k as i32;
    let mut i = 0;
    while 0 < k && i < arr.len() - 1 {
        let mut j = i + 1;
        while j < arr.len() && arr[j] - arr[i] < target {
            j += 1;
        }

        if j >= arr.len() {
            return false;
        }

        k -= 1;
        i = j;
    }

    arr[arr.len() - 1] - arr[i] >= target
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n]
    }

    let arr = std::iter::once(0)
        .chain(a.into_iter())
        .chain(std::iter::once(l))
        .collect::<Vec<usize>>();

    let mut r = (l / (k + 1)) + 1;
    let mut l = 1;

    while l + 1 < r {
        let mid = (r + l) / 2;
        if check(&arr, k, mid) {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l);
}
