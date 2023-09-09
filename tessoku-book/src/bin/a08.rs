use proconio::input;

fn main() {
    input! {
        H: usize, // 縦マス
        W: usize, // 横マス
        X: [[i32; W]; H], // 書かれている整数X
        Q: usize, // 質問数
        ABCD: [[i32; 4]; Q], // (A行,B列)(C行,D列)
    }

    // (A行,B列)(C行,D列)の長方形領域に書かれた整数の総和
    //
    // for y in 0..H
    // {
    //     for x in 0..W
    //     {
    //         println!("X[y][x]{}", X[y][x]);
    //     }
    // }

    // 各行の累積和
    let mut delta_row = vec![vec![0; W]; H];
    for y in 0..H
    {
        delta_row[y][0] = X[y][0];
        for x in 1..W
        {
            let num = X[y][x];
            delta_row[y][x] = delta_row[y][x - 1] + num;
        }
    }

    // for y in 0..H
    // {
    //     for x in 0..W
    //     {
    //         println!("delta_row y{} x{} {}", y, x, delta_row[y][x]);
    //     }
    // }

    // 各行の累積和の列ごとの累積和
    let mut delta_column = vec![vec![0; W]; H];

    for x in 0..W
    {
        delta_column[0][x] = delta_row[0][x];
        for y in 1..H
        {
            delta_column[y][x] = delta_column[y - 1][x] + delta_row[y][x];
        }
    }

    // for y in 0..H
    // {
    //     for x in 0..W
    //     {
    //         println!("delta_column y{} x{} {}", y, x, delta_column[y][x]);
    //     }
    // }

    let mut total = 0;
    for i in 0..Q
    {
        let top_y = (ABCD[i][0] - 1) as usize;
        let left_x = (ABCD[i][1] - 1) as usize;
        let bottom_y = (ABCD[i][2] - 1) as usize;
        let right_x = (ABCD[i][3] - 1) as usize;

        // println!("top_y{}", top_y);
        // println!("left_x{}", left_x);
        // println!("bottom_y{}", bottom_y);
        // println!("right_x{}", right_x);

        // println!("delta_column[top_y][left_x]{}", delta_column[top_y][left_x]);
        // println!("delta_column[bottom_y][right_x]{}", delta_column[bottom_y][right_x]);
        let mut total = delta_column[bottom_y][right_x];
        // println!("delta_column[bottom_y][right_x]{}", delta_column[bottom_y][right_x]);
        if top_y > 0
        {
            total -= delta_column[top_y - 1][right_x];
            // println!("delta_column[top_y][right_x]{}", delta_column[top_y - 1][right_x]);
        }

        if left_x > 0
        {
            total -= delta_column[bottom_y][left_x - 1];
            // println!("delta_column[bottom_y][left_x]{}", delta_column[bottom_y][left_x - 1]);
        }


        if top_y > 0 && left_x > 0 {
            total += delta_column[top_y - 1][left_x - 1];
            // println!("delta_column[top_y][left_x]{}", delta_column[top_y - 1][left_x - 1]);
        }


        println!("{}", total);
    }
}
