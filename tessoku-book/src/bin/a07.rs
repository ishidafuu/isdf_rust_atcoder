use proconio::input;

fn main() {
    input! {
        D: usize, // D日間
        N: usize, // N人
        LR: [[usize; 2]; N], // L日目からR日目まで出席
    }

    // 各日の出席者数を出力

    // 各日の人数変化量
    let mut delta = vec![0; D];

    for i in 0..N
    {
        let in_day_index = LR[i][0] - 1;
        let out_day_index = LR[i][1];
        // println!("in_day{}", in_day);
        // println!("out_day{}", out_day);
        delta[in_day_index] += 1;
        if out_day_index < D {
            delta[out_day_index] -= 1;
        }
    }


    let mut total = 0;
    for i in 0..D
    {
        // println!("delta[i]{}", delta[i]);
        total += delta[i];

        println!("{}", total);
    }
}
