The data in the csv has several important parts. 
ID is the player ID, 
GAME_NAME is the games name,
BEHAVIOUR is actually irrelevant in that we can get the value by looking at the next parameter except in the case of the player having played exactly one hour,
PLAY_PURCHASE is the number of hours the player has played the game or 1.0 if the player has bought the and not played it or played exactly one hour,
NONE is truly irrelevant.

I assume the same player (by id) won't have played the same game twice for different amounts of time. It doesn't seem like there are cases of this, and can therefore be avoided. I think it updated the sheet every time you play the game, and just adds another entry if you start playing it.