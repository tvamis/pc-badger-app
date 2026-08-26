import data.matches

print("Welcome to the PC Badger App!")
player_name = input("What is your name?")
player_club = input(f"Hi {player_name}, what club do you play for?")
yes_no = input(f"Would you like to see the stats on who {player_club} have next?")
if yes_no == "yes":
  match = data.matches.get_match_data()
  print(f"Alright then {player_name}, let's see who {player_club} have this weekend! Loading..{match}");
elif yes_no == "no":
  print(f"No wakkas {player_name}, hope you and {player_club} go well this year!")
else:
  print(f"Sorry {player_name}, please restart!")