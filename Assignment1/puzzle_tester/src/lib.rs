use rand::Rng;

pub fn randomize_board (setup_board: &mut [i32]) {
    let mut rng = rand::thread_rng();
    for i in (0..16).rev() {
        let j: usize = rng.gen_range(0..=15);
        let temp: i32 = setup_board[i];
        setup_board[i] = setup_board[j];
        setup_board[j] = temp;
    }
}

pub fn is_solvable(setup_board: &mut [i32]) -> bool {
    //naive approach for ease of implementation
    let mut cnt: i32 = 0;
    for i in 0..15 {
        for j in (i + 1)..16 {
            if setup_board[i] > setup_board[j] {
                cnt += 1;
            }
        }
    }
    return cnt % 2 == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_board() {
        let mut setup_board: [i32;16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        assert_eq!(is_solvable(&mut setup_board), true);
    }
    #[test]
    fn test_vertical_board() {
        let mut setup_board: [i32;16] = [15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,16];
        assert_eq!(is_solvable(&mut setup_board), false);
    }
    #[test]
    fn test_samlloyd_board() {
        let mut setup_board: [i32;16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,15,13,16];
        assert_eq!(is_solvable(&mut setup_board), false);
    }
    #[test]
    fn test_other_board() {
        let mut setup_board: [i32;16] = [1,2,3,4,12,13,14,5,11,16,15,6,10,9,8,7];
        assert_eq!(is_solvable(&mut setup_board), false);
    }
    //----------------------------------------------------------
    #[test]
    fn test_board_randomization() {
        for _i in 0..10 {
            let mut setup_board: [i32;16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
            randomize_board(&mut setup_board);
            setup_board.sort();
            assert_eq!(setup_board, [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        }
    }
}
