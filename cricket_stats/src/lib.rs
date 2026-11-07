pub fn runs_needed(target: u32, scored: u32) -> Result<u32, String> {
    if target > scored {
        Ok(target - scored)
    } else {
        Err(String::from("They chased it down to win!"))
    }
}

pub fn required_rate(target: u32, scored: u32, overs_left: u32) -> Result<u32, String> {
    let runs = runs_needed(target, scored)?;
    Ok(runs / overs_left)
}

pub fn top_score(innings: u32, best: u32) -> Option<u32> {
    match innings {
        0 => None,
        _ => Some(best),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn runs_needed_gives_the_gap() {
        assert_eq!(runs_needed(150, 120), Ok(30));
    }
    #[test]
    fn runs_needed_chased_down() {
        assert_eq!(
            runs_needed(120, 150),
            Err(String::from("They chased it down to win!"))
        );
    }
    #[test]
    fn required_rate_live_game() {
        assert_eq!(required_rate(200, 100, 20), Ok(5));
    }
    #[test]
    fn required_rate_finished_game() {
        assert_eq!(
            required_rate(200, 201, 5),
            Err(String::from("They chased it down to win!"))
        );
    }
    #[test]
    fn top_score_more_than_one_game() {
        assert_eq!(top_score(10, 172), Some(172));
    }
    #[test]
    fn top_score_no_games_played() {
        assert_eq!(top_score(0, 0), None);
    }
}
