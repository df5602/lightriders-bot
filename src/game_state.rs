use std::str::FromStr;

const FIELD_WIDTH: usize = 16;
const FIELD_HEIGHT: usize = 16;

#[derive(Debug, PartialEq)]
pub enum CellType {
    Empty,
    Wall,
    PosPlayer0,
    PosPlayer1,
}

#[derive(Debug, PartialEq)]
pub struct GameState {
    pub pos_player_0: (usize, usize),
    pub pos_player_1: (usize, usize),
    field: Vec<CellType>,
}

impl FromStr for GameState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut idx_player_0 = None;
        let mut idx_player_1 = None;
        let field: Vec<CellType> = s
            .split(',')
            .enumerate()
            .map(|(i, substr)| match substr {
                "." => CellType::Empty,
                "x" => CellType::Wall,
                "0" => {
                    if idx_player_0.is_none() {
                        idx_player_0 = Some(i);
                    } else {
                        panic!("Multiple positions for player 0!");
                    }
                    CellType::PosPlayer0
                }
                "1" => {
                    if idx_player_1.is_none() {
                        idx_player_1 = Some(i);
                    } else {
                        panic!("Multiple positions for player 1!");
                    }
                    CellType::PosPlayer1
                }
                s => panic!("Cannot parse cell type: {}", s),
            })
            .collect();

        assert_eq!(FIELD_HEIGHT * FIELD_WIDTH, field.len());

        let pos_player_0 = match idx_player_0 {
            Some(idx) => {
                let y = idx / FIELD_WIDTH;
                let x = idx - FIELD_WIDTH * y;
                assert!(
                    x < FIELD_WIDTH && y < FIELD_HEIGHT,
                    "Position of player 0 is not within field!"
                );
                (x, y)
            }
            None => panic!("Cannot parse position of player 0!"),
        };

        let pos_player_1 = match idx_player_1 {
            Some(idx) => {
                let y = idx / FIELD_WIDTH;
                let x = idx - FIELD_WIDTH * y;
                assert!(
                    x < FIELD_WIDTH && y < FIELD_HEIGHT,
                    "Position of player 1 is not within field!"
                );
                (x, y)
            }
            None => panic!("Cannot parse position of player 1!"),
        };

        Ok(GameState {
            pos_player_0,
            pos_player_1,
            field,
        })
    }
}