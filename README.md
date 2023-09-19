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
kill to earn items, and other players who you can kill to earn points. Most
points at the end of 5 minutes wins!

## Advanced Topic Description

### Networking

UDP networking to connect together players with a listen server on the host
player's computer. Connecting over LAN directly by IP. Focus on reliability
and performance.
    
### Procedural Generation

Each round starts with a randomly generated arena, placing enemy camps, items,
decorations, shops, obstacles, and terrain throughout the map. Focus on balance,
complexity, and natural appearance.

## Midterm Goals

* Networking: Players can see each other in a lobby 
* ProcGen: Gameplay-complete maps should be produced, not necessarily good. Basic minimap.
* Gameplay: Sword should work to do damage, enemies should be able to kill you
* Scoring: Score system and timer work
* UI supports currently built features, game doesn't irreperably crash.

## Final Goals

* 25%: Networking: Complete listen server, network should not be an issue for gameplay
* 25%: ProcGen: Maps are varied and natural, multiple camp types and decorations
* 15%: Gameplay: Sword combat finished, at least one extra ability, upgrades work, enemies can kill you and some have abilities too.
* 5%: Scoring: Working leaderboard with statistics in addition to scoring and timer.
* 5%: UI supports all features and has visual/auditory feedback
* 5%: Game runs smoothly and all abilities work together without bugs.

## Stretch Goals

* Rollback and prediction netcode
* _Epic_ boss battle
