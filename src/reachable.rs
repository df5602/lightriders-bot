// Algorithm adapted from:
// http://www.adammil.net/blog/v126_A_More_Efficient_Flood_Fill.html

use game_state::{CellType, GameState};

#[derive(PartialEq)]
enum CellReachability {
    Empty,
    Blocked,
    Reachable,
}

pub fn count_reachable_cells(game_state: &GameState, x: usize, y: usize) -> usize {
    if game_state.at(x, y) == CellType::Empty {
        let mut cells: Vec<CellReachability> = game_state
            .field
            .iter()
            .map(|cell| match cell {
                CellType::Empty => CellReachability::Empty,
                CellType::PosPlayer0 | CellType::PosPlayer1 | CellType::Wall => {
                    CellReachability::Blocked
                }
            })
            .collect();

        flood_fill(
            &mut cells,
            x,
            y,
            game_state.width(),
            game_state.height(),
            true,
        );

        eprintln!("+----------------+");
        for (i, cell) in cells.iter().enumerate() {
            if i % 16 == 0 {
                eprint!("|");
            }
            if y * 16 + x == i {
                eprint!("@");
            } else {
                match cell {
                    CellReachability::Empty => eprint!(" "),
                    CellReachability::Blocked => eprint!("."),
                    CellReachability::Reachable => eprint!("#"),
                }
            }
            if i % 16 == 15 {
                eprintln!("|")
            }
        }
        eprintln!("+----------------+");

        cells
            .iter()
            .filter(|&cell| *cell == CellReachability::Reachable)
            .count()
    } else {
        0
    }
}

fn flood_fill(
    cells: &mut [CellReachability],
    mut x: usize,
    mut y: usize,
    width: usize,
    height: usize,
    expand_top_left: bool,
) {
    if expand_top_left {
        // move to top left as far as possible
        loop {
            let ox = x;
            let oy = y;

            while y != 0 && cells[(y - 1) * width + x] == CellReachability::Empty {
                y -= 1;
            }

            while x != 0 && cells[y * width + x - 1] == CellReachability::Empty {
                x -= 1;
            }

            if x == ox && y == oy {
                break;
            }
        }
    }

    let mut last_row_length = 0;

    loop {
        let mut row_length = 0;
        let mut sx = x;

        // handle row starting later than previous row
        if last_row_length != 0 && cells[y * width + x] != CellReachability::Empty {
            loop {
                last_row_length -= 1;
                if last_row_length == 0 {
                    return;
                }
                x += 1;
                if cells[y * width + x] == CellReachability::Empty {
                    break;
                }
            }
            sx = x;
        } else {
            // handle row starting earlier than previous row
            while x != 0 && cells[y * width + x - 1] == CellReachability::Empty {
                x -= 1;
                cells[y * width + x] = CellReachability::Reachable;

                if y != 0 && cells[(y - 1) * width + x] == CellReachability::Empty {
                    // handle area above new found reachable cell
                    flood_fill(cells, x, y - 1, width, height, true);
                }

                row_length += 1;
                last_row_length += 1;
            }
        }

        // scan current row
        while sx < width && cells[y * width + sx] == CellReachability::Empty {
            cells[y * width + sx] = CellReachability::Reachable;

            row_length += 1;
            sx += 1;
        }

        // current row is shorter than previous row, more reachable area may be to the right
        if row_length < last_row_length {
            let end = x + last_row_length;

            while sx + 1 < end {
                sx += 1;
                if cells[y * width + sx] == CellReachability::Empty {
                    flood_fill(cells, sx, y, width, height, false);
                }
            }
        } else if row_length > last_row_length && y != 0 {
            // current row is longer than previous row, more reachable area may be to the right
            let mut ux = x + last_row_length;

            while ux + 1 < sx {
                ux += 1;
                if cells[(y - 1) * width + ux] == CellReachability::Empty {
                    flood_fill(cells, ux, y - 1, width, height, true);
                }
            }
        }

        last_row_length = row_length;

        y += 1;
        if last_row_length == 0 || y >= height {
            break;
        }
    }
}
