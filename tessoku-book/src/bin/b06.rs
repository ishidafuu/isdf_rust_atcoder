use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32;n],
        q: usize,
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
        let total = (r - l + 1) as i32;
        let hit = if l == 1 { sum[r - 1] } else { sum[r - 1] - sum[l - 2] };
        let miss = (total - hit) as i32;
        // println!("total{}", total);
        // println!("sum[r - 1]{}", sum[r - 1]);
        // if l != 1 { println!("sum[l - 2]{}", sum[l - 2]); }
        // println!("hit{} miss{}", hit, miss);
        if hit > miss {
            println!("win");
        } else if hit < miss {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
