use proconio::input;

fn main() {
    input! {
        T: usize, // 閉店時間
        N: usize, // N人
        LR: [[usize; 2]; N], // 出勤時間L　退勤時間R
    }

    // 0からT-1時までの時刻t t+0.5で何人働いているか

    // 各時間ごとの変化人数(T時も含む)
    let mut delta = vec![0; T + 1];

    for i in 0..N
    {
        // iさんの出勤・退勤時間
        let in_time = LR[i][0];
        let out_time = LR[i][1];
        delta[in_time] += 1;
        delta[out_time] -= 1;
    }

    // 現在働いている人数
    let mut worker_count = 0;
    for i in 0..T
    {
        worker_count += delta[i];
        println!("{}", worker_count);
    }
}
