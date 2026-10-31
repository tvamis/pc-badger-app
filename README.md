# PlayCricket Badger App

# What it is

It is an interactive, cricket statistics application tailored to helping your team and you as a player prepare for opposition and to improve your game! PlayCricket is a wonderful site, I check it every week, and houses some incredible data. But it hasn't had a lick of paint for a decade, this is to bring the data that PC has and bring it to be seen to prepare for next week's fixture. It's a cricket badger's dream!

# V 0.1

This will host the data of West Wittering Cricket Club (WWCC), it will show other clubs and in future the ECB how a club's and player's data can be shown in the app. It will allow you to find your player name at your club to save to your account and the club itself, from there it will show your player data, your club data and pose the 2nd XI for WWCC as the opposition for next week to show how we will present opposition data.

# The big data points

This app will house a lot of data, but there are four areas that will be key for user engagement:

## Par score at the ground
What it shows: It shows the number of runs needed at a ground where anything more leads to a higher chance of your team winning.
Data needed: The run totals and the final result of the match at the specific ground.
How to work it out: Split the first-innings totals into two groups, totals that were defended (batting first won) and totals that were chased down (batting first lost). Average each group, add the two averages together and divide by two. This is V1, it will get more intelligent the more data comes in.
Covered by our key? For West Wittering’s home fixtures, yes. But will need either the PlayCricket API key or other teams in the league to contribute.

## Opposition form
What it shows: The opposition’s form in the last four matches leading up to your game.
Data needed: Results for that team in their last four fixtures.
How to work it out: Decided by a simple W (win), L (lost) or T (tied) icon.
Covered by our key?: For West Wittering’s fixtures, yes. But will need either the PlayCricket API key or other teams in the league to contribute.

## Key players
What it shows: Batters and bowlers of the opposition side who are in form and to pay particular attention to.
Data needed: The individual batter scores and bowling figures in fixtures that the team has played in.
How to work it out: Find players with the best batting/bowling averages, best score/figures and most runs/wickets in the league season as well as the last four games, to be alongside the form table.
Covered by our key?: For West Wittering’s fixtures, yes. But will need either the PlayCricket API key or other teams in the league to contribute.

## The Badger’s AI preview
What it shows: A summary of the stats for both sides, pointing out their strengths, pointing out weather conditions and give a build-up for the game that doesn't favour either side (even if unbeaten top of the league is facing bottom who haven't won a game!).
Data needed: All the data and more mentioned above and the weather patterns.
How to work it out: By reading the dataset of each team in their season, processing through an AI model (Claude) and then showing a playful response fitting with UK cricket banter.
Covered by our key?: We will need to have an API with the LLM needed, but this is available so can be done.

# How will it be built

The backend in this phase will largely be built in Python, this will be housed in AWS, using its various products:
- DynamoDB - house the data to output.
- Lambda - runs through python code to fetch the data and run through stat calculations.
- S3 - store raw data.
To build the application when it is ready for production. It will also be built using Terraform, to handle IaC for a more efficient and adaptable application.

# Status

Awaiting for WWCC API key for a dataset, will create dummy data sets to run through testing phases. The main backend and frontend is still being built, currently at V0.0.1.