# Introduction
The famous [Bevy](https://bevyengine.org/) [Minesweeper Tutorial](https://dev.to/qongzi/bevy-minesweeper-introduction-4l7f)
adopted to Bevy 12.1 (04.11.2023). The tutorial (series of articles) is good introduction to Bevy, 
but old Bevy 0.6 version. If you try learn Bevy 12.x using tutorial, you face a challenge
of incompatible changes API. 

# Disclaimer
For me, the main difficulty in adopting tutorial was the simultaneous study of ECS, Bevy and API changes.
Due to it I have little difference between original tutorial and my solution not only in API. I didn't implement adaptive size of tiles and 
full support of [bevy-inspector-egui](http://crates.io/crates/bevy-inspector-egui) implemented later (Chapter 6 and 7) than in the tutorial.

Also I suppose that some  techniques from the tutorial is outdated (for ex., layout of tiles). This repository close to original tutorial.
# Chapters and commits
| Article                                                                                                                                                  | Commit                                                                                                          | Full source                                                                                                  |
| -------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| [02 Project setup ](https://dev.to/qongzi/bevy-minesweeper-part-1-534c)                                                                                  | [f95f83b](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/f95f83b97d6b34b193cdab3080c6a965ed3f3b9a) | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/f95f83b97d6b34b193cdab3080c6a965ed3f3b9a) |
| [03 Tile Map Generation](https://dev.to/qongzi/bevy-minesweeper-part-2-1hi5)                                                                             | [45e742b](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/45e742b4cab3aab62bb263cb3d366ae9ce006c45)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/45e742b4cab3aab62bb263cb3d366ae9ce006c45) |
| [04 The Board](https://dev.to/qongzi/bevy-minesweeper-part-3-1a9a)                                                                                       | [ab2518b](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/ab2518b46abeccc76a790ff6602667236ccf3d97)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/ab2518b46abeccc76a790ff6602667236ccf3d97) |
| [05 Tiles and Components ](https://dev.to/qongzi/bevy-minesweeper-part-4-2co9)                                                                           | [ef61ba9](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/ef61ba9d23d316ca6cb629c63b6cbddcb0eba96a)                                                                                                         | [source](ef61ba9d23d316ca6cb629c63b6cbddcb0eba96a)                                                           |
| [06 Input Management](https://dev.to/qongzi/bevy-minesweeper-part-5-24j4) <br> [07 Uncovering tiles](https://dev.to/qongzi/bevy-minesweeper-part-6-46jh) | [4e98b51](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/4e98b5140ebc19c2423d78be081ccac9b3fd0c9d)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/4e98b5140ebc19c2423d78be081ccac9b3fd0c9d) |
|                                                                                                                                                          |
| [08 Safe Start](https://dev.to/qongzi/bevy-minesweeper-part-7-1ko2)                                                                                      | [35bc7a7](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/35bc7a73901d639db09243b7bd61ba9873dfd4bd)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/35bc7a73901d639db09243b7bd61ba9873dfd4bd) |
| [09 Generic States](https://dev.to/qongzi/bevy-minesweeper-part-8-4apn)                                                                                  | [6229aca](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/6229aca4282ce473f38bcb3193c40a2bd33e520a)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/6229aca4282ce473f38bcb3193c40a2bd33e520a) |
| [10 Assets](https://dev.to/qongzi/bevy-minesweeper-part-9-534e)                                                                                          | [6ecf9e6](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/6ecf9e60fca06f7fa5384becade743c042c41b25)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/6ecf9e60fca06f7fa5384becade743c042c41b25) |
| [11 Marking Tiles](https://dev.to/qongzi/bevy-minesweeper-part-10-5hie)                                                                                  | [ddadba3](https://github.com/leonidv/bevy-minesweeper-tutorial/commit/ddadba3eec38a1ca6066316ac94c8b5ce2eb052a)                                                                                                         | [source](https://github.com/leonidv/bevy-minesweeper-tutorial/tree/ddadba3eec38a1ca6066316ac94c8b5ce2eb052a) |


# Chapter 9. Generic states
Very different from the tutorial, because Bevy made many changes to the concept of systems.

My code implements scenarios:
* Generates new board if player press `G` key.
* Pause game if player press `P` key. (Game board is hidden and click is ignored).

All the same, the code shows how to:
1. Decouples application states and plugin states.
2. Runs systems (process events) only in certain states.
3. Switches from one state to another state.