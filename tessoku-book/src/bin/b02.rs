use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if a == 0 {
        println!("No");
    }

    for i in a..=b {
        if 100 % i == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
