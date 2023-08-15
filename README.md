# anime
A telegram bot made in rust that searches for anime watch order's in its data base.

There are 3 commands:

1-/help this command lists the commands for the telegram bot and the text for this command is written in the file named resources/help.txt if you change this file it will change what get's sent to the user

2-/list this command list the available anime in the database which is located in resources/anime_list this file doesn't get updated automatically

3-/search this command take the aruguement's after itself and remove any spaces and then turn's it into lowercase and checks the resources/anime_list for a file that matches its name and send's that to the user after a bit of formating

that is all.(it also responds to hi or hello)
