use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // rows
    let valid_rows = board.iter().all(check_line);

    // cols
    let valid_cols =
        (0..board.len()).all(|i| check_line(&board.iter().map(|row| row[i]).collect::<Vec<_>>()));

    // squares
    let sq_dim = 3;
    let valid_squares = (0..sq_dim).all(|i| {
        (0..sq_dim).all(|j| {
            check_line(
                &board[i * 3..i * 3 + 3]
                    .iter()
                    .flat_map(|row| &row[j * 3..j * 3 + 3])
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
    });

    valid_rows && valid_cols && valid_squares
}

fn check_line(line: &Vec<char>) -> bool {
    // filter out empties ('.')
    let xs: Vec<&char> = line.iter().filter(|&&ch| ch != '.').collect();

    xs.len() == xs.into_iter().collect::<HashSet<&char>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            true
        );

        assert_eq!(
            is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
    }

    #[test]
    fn t_check_line() {
        assert_eq!(
            check_line(&vec!['8', '6', '.', '8', '4', '7', '.', '.', '.']),
            false
        );
        assert_eq!(
            check_line(&vec!['6', '.', '.', '1', '9', '5', '.', '.', '.']),
            true
        );
    }
}
