use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [[usize; 4]; n], //a以上c以下のx、b以上d以下のy
    }

    // println!("h {}", h);
    // println!("w {}", w);
    // println!("n {}", n);
    // println!("abcd");
    // for row in &abcd {
    //     println!("{:?}", row);
    // }

    let mut delta_map = vec![vec![0 as i32; w]; h];


    for i in 0..n
    {
        // インデックスに変換
        let left_x = abcd[i][0] - 1;
        let top_y = abcd[i][1] - 1;
        let right_x = abcd[i][2] - 1;
        let bottom_y = abcd[i][3] - 1;

        // println!("left_x {}", left_x);
        // println!("top_y {}", top_y);
        // println!("right_x {}", right_x);
        // println!("bottom_y {}", bottom_y);

        delta_map[top_y][left_x] += 1;
        delta_map[top_y][right_x + 1] -= 1;
        delta_map[bottom_y + 1][left_x] -= 1;
        delta_map[bottom_y + 1][right_x + 1] += 1;
    }

    // println!("delta_map");
    // for row in &delta_map {
    //     println!("{:?}", row);
    // }

    // 各行ごとの累積和
    let mut delta_row = vec![vec![0; w]; h];
    for y in 0..h
    {
        delta_row[y][0] = delta_map[y][0];
        for x in 1..w
        {
            delta_row[y][x] = delta_row[y][x - 1] + delta_map[y][x];
        }
    }


    // println!("delta_row");
    // for row in &delta_row {
    //     println!("{:?}", row);
    // }

    // 各行の累積和の列ごとの累積和
    let mut delta_column = vec![vec![0; w]; h];

    for x in 0..w
    {
        delta_column[0][x] = delta_row[0][x];
        for y in 1..h
        {
            delta_column[y][x] = delta_column[y - 1][x] + delta_row[y][x];
        }
    }

    for row in &delta_column {
        let row_str: String = row.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", row_str);
    }
}
