@@lesson m2-grids Grids: Lists of Lists
@@text
Game boards, spreadsheets, images, maps, seating plans — lots of data is a **grid**: rows and columns. A grid is simply a **list of lists**: a list of rows, where each row is a list of cells.

## Building a grid

With arrays, when the size is fixed:

```rust
fn main() {
    let board = [
        ['X', 'O', 'X'],
        [' ', 'X', 'O'],
        ['O', ' ', 'X'],
    ];
    println!("Middle cell: {}", board[1][1]);
    println!("Top right: {}", board[0][2]);
}
```

`board[row][col]`: first pick the row, then the column within that row. Both start at 0.

```text
          col 0  col 1  col 2
 row 0  [  'X',   'O',   'X' ]
 row 1  [  ' ',   'X',   'O' ]
 row 2  [  'O',   ' ',   'X' ]
```

With vectors, when the size is decided while the program runs:

```rust
fn main() {
    let rows = 3;
    let cols = 5;
    let mut grid = vec![vec!['.'; cols]; rows];   // 3 rows of 5 dots
    grid[1][2] = '#';
    for row in &grid {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}
```

`vec![vec!['.'; cols]; rows]` makes `rows` copies of a row of `cols` dots.

## Visiting every cell: nested loops

To go through a grid, use a loop over rows with a loop over columns inside:

```rust
fn main() {
    let heights = [
        [1, 3, 2],
        [4, 9, 5],
        [2, 6, 1],
    ];
    let mut highest = heights[0][0];
    let mut at = (0, 0);
    for r in 0..heights.len() {
        for c in 0..heights[r].len() {
            if heights[r][c] > highest {
                highest = heights[r][c];
                at = (r, c);
            }
        }
    }
    println!("Highest point {highest} at row {}, col {}", at.0, at.1);
}
```

## Checking lines: rows, columns, diagonals

In games like tic-tac-toe you check lines of cells:

```rust
fn main() {
    let b = [
        ['X', 'O', 'O'],
        ['X', 'X', 'O'],
        ['O', 'O', 'X'],
    ];
    // a column: same col, every row
    let col0_all_x = b[0][0] == 'X' && b[1][0] == 'X' && b[2][0] == 'X';
    // the main diagonal: row == col
    let diag_all_x = b[0][0] == 'X' && b[1][1] == 'X' && b[2][2] == 'X';
    println!("column 0: {col0_all_x}, diagonal: {diag_all_x}");
}
```

## Neighbours and the edges

Many grid programs look at a cell's **neighbours** (up, down, left, right). The danger: cells on the edge don't have all neighbours, and indexes are `usize`, which **can't be negative**. `0 - 1` on a `usize` panics with "attempt to subtract with overflow". So check before you step:

```rust
fn main() {
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    let (r, c) = (0, 0);
    let mut neighbours = Vec::new();
    if r > 0 { neighbours.push(grid[r - 1][c]); }                    // up
    if r + 1 < grid.len() { neighbours.push(grid[r + 1][c]); }       // down
    if c > 0 { neighbours.push(grid[r][c - 1]); }                    // left
    if c + 1 < grid[r].len() { neighbours.push(grid[r][c + 1]); }    // right
    println!("Neighbours of ({r},{c}): {:?}", neighbours);
}
```

This bounds checking appears in every grid game you'll ever write — including GUI games later.

@@quiz
In `grid[2][5]`, what does the `2` select?
* The row (the third row)
- The column
- The cell number 2 counting from the top-left
? First index = row, second index = column.

@@quiz
`let r: usize = 0;` — what happens with `r - 1`?
- It gives -1
* It panics (or is a compile error), because `usize` can't be negative
- It gives 0
? Check `if r > 0` before subtracting from an index.

@@exercise Tic-tac-toe winner
Write `winner`, which looks at a 3×3 board and returns the winner: `'X'`, `'O'`, or `' '` (a space) if nobody has won. A player wins with three of their marks in a row, column or diagonal.
@@starter
fn winner(b: [[char; 3]; 3]) -> char {
    todo!()
}

fn main() {
    let board = [
        ['X', 'O', 'O'],
        ['X', 'X', 'O'],
        ['O', 'O', 'X'],
    ];
    println!("Winner: {:?}", winner(board));
}
@@tests
#[test]
fn checks() {
    assert_eq!(winner([['X', 'O', 'O'], ['X', 'X', 'O'], ['O', 'O', 'X']]), 'X', "diagonal");
    assert_eq!(winner([['O', 'O', 'O'], ['X', 'X', ' '], [' ', ' ', 'X']]), 'O', "top row");
    assert_eq!(winner([['X', 'O', ' '], ['X', 'O', ' '], [' ', 'O', 'X']]), 'O', "middle column");
    assert_eq!(winner([[' ', ' ', 'X'], [' ', 'X', ' '], ['X', 'O', 'O']]), 'X', "other diagonal");
    assert_eq!(winner([['X', 'O', 'X'], ['X', 'O', 'O'], ['O', 'X', 'X']]), ' ', "draw");
    assert_eq!(winner([[' '; 3]; 3]), ' ', "an empty board has no winner");
}
@@hint
Loop `i` from 0 to 2: row `i` is a win if `b[i][0] != ' ' && b[i][0] == b[i][1] && b[i][1] == b[i][2]`. Columns are similar with the indexes swapped. Then check the two diagonals. Don't forget the `!= ' '` check — three empty cells aren't a win!
@@solution
fn winner(b: [[char; 3]; 3]) -> char {
    for i in 0..3 {
        if b[i][0] != ' ' && b[i][0] == b[i][1] && b[i][1] == b[i][2] {
            return b[i][0];
        }
        if b[0][i] != ' ' && b[0][i] == b[1][i] && b[1][i] == b[2][i] {
            return b[0][i];
        }
    }
    if b[1][1] != ' ' && b[0][0] == b[1][1] && b[1][1] == b[2][2] {
        return b[1][1];
    }
    if b[1][1] != ' ' && b[0][2] == b[1][1] && b[1][1] == b[2][0] {
        return b[1][1];
    }
    ' '
}

fn main() {
    let board = [
        ['X', 'O', 'O'],
        ['X', 'X', 'O'],
        ['O', 'O', 'X'],
    ];
    println!("Winner: {:?}", winner(board));
}

@@exercise Minesweeper hints
In Minesweeper, each empty cell shows how many mines touch it — including **diagonally** (up to 8 neighbours). Write `mine_hints`, which takes a field where `'*'` is a mine and `'.'` is empty, and returns a grid of the same size where mines stay `'*'` and empty cells become the digit of the number of neighbouring mines (`'0'` to `'8'`).

```text
*...      *100
....  →   2210
.*..      1*10
```

Tip: `char::from_digit(n, 10).unwrap()` turns a number 0–9 into a digit character.
@@starter
fn mine_hints(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    todo!()
}

fn main() {
    let field = vec![
        vec!['*', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '*', '.', '.'],
    ];
    for row in mine_hints(&field) {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}
@@tests
#[test]
fn checks() {
    let to_grid = |rows: &[&str]| -> Vec<Vec<char>> { rows.iter().map(|r| r.chars().collect()).collect() };
    assert_eq!(mine_hints(&to_grid(&["*...", "....", ".*.."])), to_grid(&["*100", "2210", "1*10"]));
    assert_eq!(mine_hints(&to_grid(&["***", "*.*", "***"])), to_grid(&["***", "*8*", "***"]), "surrounded by 8 mines");
    assert_eq!(mine_hints(&to_grid(&["."])), to_grid(&["0"]), "a single empty cell");
}
@@hint
Copy the field (`let mut result = field.clone();`). For each empty cell `(r, c)`, check all 8 directions: loop `dr` over `-1..=1` and `dc` over `-1..=1` as `i32`s, skip `(0, 0)`, compute `nr = r as i32 + dr` and `nc = c as i32 + dc`, and skip it if it's outside the grid (`< 0` or `>=` the size). Otherwise, if `field[nr as usize][nc as usize] == '*'`, count it.
@@solution
fn mine_hints(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = field.clone();
    let rows = field.len() as i32;
    for r in 0..field.len() {
        let cols = field[r].len() as i32;
        for c in 0..field[r].len() {
            if field[r][c] == '*' {
                continue;
            }
            let mut count = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nc < 0 || nr >= rows || nc >= cols {
                        continue;
                    }
                    if field[nr as usize][nc as usize] == '*' {
                        count += 1;
                    }
                }
            }
            result[r][c] = char::from_digit(count, 10).unwrap();
        }
    }
    result
}

fn main() {
    let field = vec![
        vec!['*', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '*', '.', '.'],
    ];
    for row in mine_hints(&field) {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}
