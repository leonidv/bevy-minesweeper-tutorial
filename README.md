## Chapter 8. Generic states
Very different from the tutorial, because Bevy made many changes to the concept of systems.

My code implements scenarios:
* Generates new board if player press `G` key.
* Pause game if player press `P` key. (Game board is hidden and click is ignored).

All the same, the code shows how to:
1. Decouples application states and plugin states.
2. Runs systems (process events) only in certain states.
3. Switches from one state to another state.