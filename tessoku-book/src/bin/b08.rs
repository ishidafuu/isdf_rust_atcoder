use proconio::input;

fn main() {
    input! {
        n: usize, // N個の点
        xy: [[usize; 2]; n], //x,y
        q: usize,
        abcd: [[usize; 4]; q], //a以上c以下のx、b以上d以下のy
    }


    let mut min_x = xy[0][0];
    let mut min_y = xy[0][1];
    let mut max_x = xy[0][0];
    let mut max_y = xy[0][1];
    for i in 0..n
    {
        let x = xy[i][0];
        let y = xy[i][1];
        // println!("x{}", x);
        // println!("y{}", y);

        if x > max_x {
            max_x = x;
        } else if x < min_x {
            min_x = x;
        }

        if y > max_y {
            max_y = y;
        } else if y < min_y {
            min_y = y;
        }
    }

    // println!("max_x{}", max_x);
    // println!("min_x{}", min_x);
    // println!("max_y{}", max_y);
    // println!("min_y{}", min_y);

    let x_len = max_x - min_x + 1;
    let y_len = max_y - min_y + 1;

    // println!("x_len{}", x_len);
    // println!("y_len{}", y_len);

    let mut delta_map = vec![vec![0 as usize; x_len]; y_len];

    for i in 0..n
    {
        let x = xy[i][0] - min_x;
        let y = xy[i][1] - min_y;
        // println!("x{} y{}", x, y);
        delta_map[y][x] += 1;
    }

    // for y in 0..y_len
    // {
    //     for x in 0..x_len
    //     {
    //         println!("x{} y{} delta_map[y][x] {}", x, y, delta_map[y][x]);
    //     }
    // }

    // println!("{}", total);

    // 各行ごとの累積和
    let mut delta_row = vec![vec![0; x_len]; y_len];
    for y in 0..y_len
    {
        delta_row[y][0] = delta_map[y][0];
        for x in 1..x_len
        {
            delta_row[y][x] = delta_row[y][x - 1] + delta_map[y][x];
            // println!("delta_row y{} x{} {}", y, x, delta_row[y][x]);
        }
    }

    // 各行の累積和の列ごとの累積和
    let mut delta_column = vec![vec![0; x_len]; y_len];

    for x in 0..x_len
    {
        delta_column[0][x] = delta_row[0][x];
        for y in 1..y_len
        {
            delta_column[y][x] = delta_column[y - 1][x] + delta_row[y][x];
        }
    }

    // for y in 0..x_len
    // {
    //     for x in 0..y_len
    //     {
    //         println!("delta_column y{} x{} {}", y, x, delta_column[y][x]);
    //     }
    // }

    // println!("{:?}", delta_column);

    // println!("delta_map");
    // for row in &delta_map {
    //     println!("{:?}", row);
    // }
    //
    // println!("delta_row");
    // for row in &delta_row {
    //     println!("{:?}", row);
    // }
    //
    // println!("delta_column");
    // for row in &delta_column {
    //     println!("{:?}", row);
    // }
    //
    //
    // println!("abcd");
    // for row in &abcd {
    //     println!("{:?}", row);
    // }

    let mut total = 0;
    for i in 0..q
    {
        // println!("i{}", i);
        // println!("min_x{}", min_x);
        // println!("min_y{}", min_y);
        // println!("{}", acbd[i][0]);
        // println!("{}", acbd[i][1]);
        // println!("{}", acbd[i][2]);
        // println!("{}", acbd[i][3]);

        let left_x = if abcd[i][0] < min_x { min_x - min_x } else { (abcd[i][0] - min_x) as usize };
        let top_y = if abcd[i][1] < min_y { min_y - min_y } else { (abcd[i][1] - min_y) as usize };
        let right_x = if abcd[i][2] > max_x { max_x - min_x } else { (abcd[i][2] - min_x) as usize };
        let bottom_y = if abcd[i][3] > max_y { max_y - min_y } else { (abcd[i][3] - min_y) as usize };
        //
        // println!("max_x{}", max_x);
        // println!("min_x{}", min_x);
        // println!("max_y{}", max_y);
        // println!("min_y{}", min_y);
        // println!("abcd[i][0]{}", abcd[i][0]);
        // println!("abcd[i][1]{}", abcd[i][1]);
        // println!("abcd[i][2]{}", abcd[i][2]);
        // println!("abcd[i][3]{}", abcd[i][3]);
        //
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
