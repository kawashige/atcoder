use proconio::input;

fn main() {
    input! {
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32,
    }

    let s = std::iter::repeat('R')
        .take((tx - sx) as usize)
        .chain(std::iter::repeat('U').take((ty - sy) as usize))
        .chain(std::iter::repeat('L').take((tx - sx) as usize))
        .chain(std::iter::repeat('D').take((ty - sy + 1) as usize))
        .chain(std::iter::repeat('R').take((tx - sx + 1) as usize))
        .chain(std::iter::repeat('U').take((ty - sy + 1) as usize))
        .chain(std::iter::once('L'))
        .chain(std::iter::once('U'))
        .chain(std::iter::repeat('L').take((tx - sx + 1) as usize))
        .chain(std::iter::repeat('D').take((ty - sy + 1) as usize))
        .chain(std::iter::once('R'))
        .collect::<String>();

    print!("{}", s);
}
