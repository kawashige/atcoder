use proconio::input;
use proconio::marker::Chars;

fn is_palindrome(nums: &[char]) -> bool {
    let l = nums.len();
    for i in 0..(l / 2) {
        if nums[i] != nums[l - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        s: Chars
    }

    let l = s.len();
    if is_palindrome(&s[0..])
        && is_palindrome(&s[..((l - 1) / 2)])
        && is_palindrome(&s[(((l + 3) / 2) - 1)..])
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
