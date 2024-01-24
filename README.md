### What's this

Some quick PoC I wrote for [this leetcode discussion](https://leetcode.com/discuss/interview-question/system-design/4615686/phonepe-sde2-machine-coding-round-24-hours)
This was done in whatever could be achieved in the timeframe of 1 hour.
Feel free to PR.

### Usage

```
board:
  width: 10
  height: 10
  supermoves:
    15: !Snake 5
    5: Crocodile
    10: !Ladder 5
    12: !Mine 2
players:
- name: player1
  pos:
    pos: 0
    state: Normal
- name: player2
  pos:
    pos: 0
    state: Normal
dice:
  count: 1
  strategy: Max
  die:
    sides: 6

Player { name: "player1", pos: Position { pos: 0, state: Normal } } roll 5
Player { name: "player2", pos: Position { pos: 0, state: Normal } } roll 5
------------------------
Player { name: "player1", pos: Position { pos: 0, state: Normal } } roll 1
Player { name: "player2", pos: Position { pos: 0, state: Normal } } roll 6
------------------------
Player { name: "player1", pos: Position { pos: 1, state: Normal } } roll 4
Player { name: "player2", pos: Position { pos: 6, state: Normal } } roll 2
------------------------
Player { name: "player1", pos: Position { pos: 0, state: Normal } } roll 6
Player { name: "player2", pos: Position { pos: 8, state: Normal } } roll 5
------------------------
Player { name: "player1", pos: Position { pos: 6, state: Normal } } roll 1
Player { name: "player2", pos: Position { pos: 13, state: Normal } } roll 1
------------------------
Player { name: "player1", pos: Position { pos: 7, state: Normal } } roll 5
Player { name: "player2", pos: Position { pos: 14, state: Normal } } roll 4
------------------------
Player { name: "player1", pos: Position { pos: 12, state: OnMine(2) } } roll 4
Player { name: "player2", pos: Position { pos: 18, state: Normal } } roll 2
------------------------
Player { name: "player1", pos: Position { pos: 12, state: OnMine(1) } } roll 4
Player { name: "player2", pos: Position { pos: 20, state: Normal } } roll 3
------------------------
Player { name: "player1", pos: Position { pos: 12, state: OnMine(0) } } roll 5
Player { name: "player2", pos: Position { pos: 23, state: Normal } } roll 6
------------------------
Player { name: "player1", pos: Position { pos: 17, state: Normal } } roll 5
Player { name: "player2", pos: Position { pos: 29, state: Normal } } roll 4
------------------------
board:
  width: 10
  height: 10
  supermoves:
    15: !Snake 5
    5: Crocodile
    10: !Ladder 5
    12: !Mine 2
players:
- name: player1
  pos:
    pos: 22
    state: Normal
- name: player2
  pos:
    pos: 33
    state: Normal
dice:
  count: 1
  strategy: Max
  die:
    sides: 6
```