use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        a:[i32;n],
    }

    for i in a {
        if i == x {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
