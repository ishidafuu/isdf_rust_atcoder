use proconio::input;

fn main() {
    input! {
        n: i32,
        a:[i32;n],
    }

    for a0 in &a {
        for a1 in &a {
            for a2 in &a {
                if a0 + a1 + a2 == 1000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
