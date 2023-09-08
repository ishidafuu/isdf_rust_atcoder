use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut a = n;
    let mut result = [0; 10];

    for i in 0..10 {
        let p = 1 << (10 - 1 - i);
        if a >= p {
            a -= p;
            result[i] = 1;
        } else {
            result[i] = 0;
        }
    }

    let result_str: Vec<String> = result.iter().map(|&n| n.to_string()).collect();
    let joined = result_str.join("");
    println!("{}", joined);
}
