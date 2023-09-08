use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32;n],
        lr: [[i32; 2]; q],
    }


    let mut sum = vec![0; n];

    sum[0] = a[0];
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i];
    }

    for i in 0..q {
        let l = lr[i][0] as usize;
        let r = lr[i][1] as usize;
        if l == 1 {
            println!("{}", sum[r - 1]);
        } else {
            let res = sum[r - 1] - sum[l - 2];
            println!("{}", res);
        }
    }
}
