use proconio::input;

fn main() {
    input! {
        n: i32,// 1..3000
        k: i32,// 3..9000
    }

    let mut count = 0;
    for r in 1..=n {
        if k > r + n + n {
            continue;
        }
        for b in 1..=n {
            let w = k - r - b;
            if w >= 1 && w <= n {
                count += 1;
            }
        }
    }


    println!("{}", count);
}
