fn main() {
    let mut arr: [[bool; 21]; 21] = [[false; 21]; 21];
    arr = addPositioningThings(arr);
    arr = addTimingPatterns(arr);
    for i in arr.iter() {
        for x in i.iter() {
            print!("{}", if x == &true { "\u{2588}\u{2588}" } else { "  " });
        }
        println!("")
    }
}

fn addPositioningThings(mut arr: [[bool; 21]; 21]) -> [[bool; 21]; 21] {
    arr = placePositioning(arr, 0, 0);
    arr = placePositioning(arr, arr.len() - 7, 0);
    arr = placePositioning(arr, 0, arr.len() - 7);
    arr
}

fn addTimingPatterns(mut arr: [[bool; 21]; 21]) -> [[bool; 21]; 21] {
    for col in 8..13 {
        arr[6][col] = col % 2 == 0;
    }

    for row in 8..13 {
        arr[row][6] = row % 2 == 0;
    }

    arr
}

fn placePositioning(
    mut arr: [[bool; 21]; 21],
    offset_x: usize,
    offset_y: usize,
) -> [[bool; 21]; 21] {
    for i in 0..7 {
        for j in 0..7 {
            if i == 0
                || i == 6
                || j == 0
                || j == 6
                || ((i == 2 || i == 4) && (j >= 2 && j <= 4))
                || ((j == 2 || j == 4) && (i >= 2 && i <= 4))
                || (i == 3 && j == 3)
            {
                arr[offset_x + i][offset_y + j] = true;
            }
        }
    }
    arr
}
