use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut a = n;
    let mut result = 0;

    for i in 0..8 {
        if a % 10 == 1 {
            result += 1 << i;
        }
        a /= 10;
    }

    println!("{}", result);
}
