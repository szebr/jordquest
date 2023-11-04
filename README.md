# Jordquest

by Red Delicious

## Team Members

* Advanced Topic Subteam 1: Networking
	* Ian Whitfield
	* Jordan Brudenell
	* Ruoheng Xu

* Advanced Topic Subteam 2: Procedural Generation
	* Sam Durigon
	* Alex Lampe
	* Brendan Szewczyk
	* Garrett DiCenzo

## Game Description

Multiplayer Hack n Slash in a randomly generated arena with PvE camps you can
kill to earn items, and other players you can kill to earn points. Most
points at the end of 5 minutes wins!

## Advanced Topic Description

### Networking

UDP networking connects together players with a listen server on the host
player's computer. Connecting over LAN directly by IP. Focus on reliability
and performance.
    
### Procedural Generation

Each round starts with a randomly generated arena, placing enemy camps, items,
decorations, shops, obstacles, and terrain throughout the map. Focus on balance,
complexity, and natural appearance.

## Midterm Goals

* Networking: Players can see each other in a lobby 
* ProcGen: One static mostly gameplay-complete map should be produced, not necessarily good. Basic minimap.
* Gameplay: Sword should work to do damage, enemies should be able to kill you
* Scoring: Score is awarded for killing enemies and a 5 minute timer ends the game when it finishes
* UI supports currently built features.

## Final Goals

* 25%: Networking: Complete listen server, network should not be an overbearing and domineering issue for gameplay
* 25%: ProcGen: Maps are generated so that they appear to be varied. They should also look somewhat natural by not repeating too many objects or entities. There should be 5 different camp types and 10 different decorations. The map size should
be roughly the size of a League of Legends jungle map. Which is the equivalent of 
roughly two football sized fields. If the players move 3.5 m/s, the map should be 160x225 m.
* 20%: Gameplay: Sword combat finished, at least one extra ability, upgrades such as increased damage or reduced damage taken work, enemies should be able to kill you and some enemies will have extra powers such as increased range or health.
* 15%: UI supports all required features such as play, ability usage, attacks, and viewing upgrades. The game has visual/auditory feedback for players' and enemies' actions as well as environmental sounds such as ambiance and background sounds.
* 15%: The game runs at an acceptable speed and all abilities work together with minimal to no bugs.

## Stretch Goals

* Lag Compensated and clientside prediction net code. Specifically this stretch goal should solve much less input lag and a fairer, more singleplayer-feeling experience. Network architecture design will have to be planned around our networked gameplay. The challenges involved in implemented lag compensated netcode include: BLAH BLAH BLAH IAN FIXIE
* Scoring: Working leaderboard with statistics, scoring, and timer. Included on the scoreboard would be the stats names, player kills, monster kills, camps captured, deaths, and k/d ratio. The overall score should increase when the player kills enemy npcs, captures bases, or eliminate enemy players.
