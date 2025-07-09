fn main() {
    let board = [[1; 15]; 15];
    println!("win: {}", check_win(&board, 7, 7, 1));
}

/// 检测五子棋坐标是否在棋盘
fn check_coordinate(row: i8, col: i8) -> bool {
    row >= 0 && row < 15 && col >= 0 && col < 15
}

/// 检查五子棋输赢
fn check_win(board: &[[i8; 15]; 15], row: i8, col: i8, player: i8) -> bool {
    // 检查方向
    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
    for (row_d, col_d) in directions {
        // 记录玩家棋子数量
        let mut count = 1;
        // 正向检测
        for i in 1..5 {
            let (r, c) = (row + row_d * i, col + col_d * i);
            if check_coordinate(r, c) && board[r as usize][c as usize] == player {
                count += 1;
            } else {
                // 边界越界，遇到其他棋子，停止检测
                break;
            }
        }
        if count >= 5 {
            return true;
        }
        // 反向检测
        for i in 1..5 {
            let (r, c) = (row + row_d * -i, col + col_d * -i);
            if check_coordinate(r, c) && board[r as usize][c as usize] == player {
                count += 1;
            } else {
                // 边界越界，遇到其他棋子，停止检测
                break;
            }
        }
        if count >= 5 {
            return true;
        }
    }
    false
}
