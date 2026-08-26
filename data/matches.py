def get_match_data():
    match = {
        "home_team": input("Enter home team name: "),
        "away_team": input("Enter away team name: "),
        "date": input("Enter match date (DD-MM-YYYY): "),
        "competition": input("Enter competition/league: "),
        "toss_won_by": input("Which team won the toss? "),
        "toss_win_choice": input("Did they choose to bat or bowl first? (batting/bowling): ").strip().lower(),
        "batting_first_team": input("Which team batted first? "),
        "bowling_first_team": input("Which team bowled first? "),
        "batting_second_team": input("Which team batted second? "),
        "bowling_second_team": input("Which team bowled second? "),
        "result": input("Enter match result: "),
    }
    return match