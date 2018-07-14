use std::str::FromStr;

use game_state::GameState;

#[derive(Debug, PartialEq)]
pub enum Input {
    Timebank(usize),
    TimePerMove(usize),
    PlayerNames(String),
    YourBot(String),
    YourBotId(usize),
    FieldWidth(usize),
    FieldHeight(usize),
    GameRound(usize),
    GameField(GameState),
    ActionMove(usize),
}

pub fn parse(input: &str) -> Input {
    let mut iter = input.split_whitespace();
    match iter.next() {
        Some("settings") => match iter.next() {
            Some("timebank") => Input::Timebank(usize::from_str(iter.next().unwrap()).unwrap()),
            Some("time_per_move") => {
                Input::TimePerMove(usize::from_str(iter.next().unwrap()).unwrap())
            }
            Some("player_names") => Input::PlayerNames(iter.next().unwrap().to_owned()),
            Some("your_bot") => Input::YourBot(iter.next().unwrap().to_owned()),
            Some("your_botid") => Input::YourBotId(usize::from_str(iter.next().unwrap()).unwrap()),
            Some("field_width") => {
                Input::FieldWidth(usize::from_str(iter.next().unwrap()).unwrap())
            }
            Some("field_height") => {
                Input::FieldHeight(usize::from_str(iter.next().unwrap()).unwrap())
            }
            Some(s) => panic!("Cannot parse unknown setting: {}", s),
            None => panic!("Missing setting!"),
        },
        Some("update") => match iter.next() {
            Some("game") => match iter.next() {
                Some("round") => Input::GameRound(usize::from_str(iter.next().unwrap()).unwrap()),
                Some("field") => {
                    Input::GameField(GameState::from_str(iter.next().unwrap()).unwrap())
                }
                Some(s) => panic!("Cannot parse unknown game update: {}", s),
                None => panic!("Missing game update!"),
            },
            Some(s) => panic!("Cannot parse unknown update: {}", s),
            None => panic!("Missing update!"),
        },
        Some("action") => match iter.next() {
            Some("move") => Input::ActionMove(usize::from_str(iter.next().unwrap()).unwrap()),
            Some(s) => panic!("Cannot parse unknown action: {}", s),
            None => panic!("Missing action!"),
        },
        Some(s) => panic!("Cannot parse unknown input: {}", s),
        None => panic!("Cannot parse empty input!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_timebank() {
        let parsed: Input = parse("settings timebank 10000");
        assert_eq!(Input::Timebank(10000), parsed);
    }

    #[test]
    fn settings_time_per_move() {
        let parsed: Input = parse("settings time_per_move 200");
        assert_eq!(Input::TimePerMove(200), parsed);
    }

    #[test]
    fn settings_player_names() {
        let parsed = parse("settings player_names a,b");
        assert_eq!(Input::PlayerNames("a,b".to_owned()), parsed);
    }

    #[test]
    fn settings_your_bot() {
        let parsed = parse("settings your_bot a");
        assert_eq!(Input::YourBot("a".to_owned()), parsed);
    }

    #[test]
    fn settings_your_botid() {
        let parsed = parse("settings your_botid 0");
        assert_eq!(Input::YourBotId(0), parsed);
    }

    #[test]
    fn settings_field_width() {
        let parsed = parse("settings field_width 16");
        assert_eq!(Input::FieldWidth(16), parsed);
    }

    #[test]
    fn settings_field_height() {
        let parsed = parse("settings field_height 16");
        assert_eq!(Input::FieldHeight(16), parsed);
    }

    #[test]
    fn update_game_round() {
        let parsed = parse("update game round 5");
        assert_eq!(Input::GameRound(5), parsed);
    }

    #[test]
    fn update_game_field() {
        let parsed = parse("update game field .,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,1,x,.,.,.,.,.,.,.,.,0,x,.,.,.,.,x,x,.,.,.,.,.,.,.,.,x,x,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.,.");
        let game_state = match parsed {
            Input::GameField(game_state) => game_state,
            _ => {
                assert!(false);
                return;
            }
        };
        assert_eq!((4, 4), game_state.pos_player_0);
        assert_eq!((10, 3), game_state.pos_player_1);
    }

    #[test]
    fn action_move() {
        let parsed = parse("action move 10000");
        assert_eq!(Input::ActionMove(10000), parsed);
    }
}
