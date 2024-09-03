The data in the csv has several important parts. 
ID is the player ID, <br>
GAME_NAME is the games name, <br>
BEHAVIOUR is actually irrelevant in that we can get the value by looking at the next parameter except in the case of the player having played exactly one hour, <br>
PLAY_PURCHASE is the number of hours the player has played the game or 1.0 if the player has bought the and not played it or played exactly one hour, <br>
NONE is truly irrelevant. <br>

I assume the same player (by id) won't have played the same game twice for different amounts of time. It doesn't seem like there are cases of this, and can therefore be avoided. <br>
I assume it updated the sheet every time you play the game, and just adds another entry if you start playing it. <br>
Since that data wasn't formatted like a normal csv, I assumed that the first comma seperated the id from the name, and the last three commas are used to seperate the last three attribues. Therefore any commas in GAME_NAME are just ignored. I only split at the first and last three commas.
I assume the data in the csv is enterred in the same way throughout. <br>