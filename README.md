# Whalesong

## Concept

- A minimal "walking simulator" (or "swimming simulator" I guess?)
- Control a whale making its way down South from Australia to the Antarctic for summer.
- (Animal migration is a cycle, get it?)
- The game is peaceful and all about the atmosphere and discovering things along the way.
- Encounters include birds, schools of fish, boats, other whales.
- There is no end, and you can't die.
  - or maybe there is an end after a while when you reach Antarctica.
- There are weather patterns and a day / night cycle. 
- There are storms
- There are icebergs
- The art is hand-sketched line art
- top down perspective
- The whale performs different actions when it encounters different things, for instance:
  - it plays around ships,
  - it eats fish,
  - it sings to other whales,
  - sometimes other whales will join ours
- The player has limited controls, but can move left and right to move towards or avoid things.
- The whale never stops swimming, just moves relentlessly South,
  - practically the whale stays still and the waves etc move around it.
- The player can possibly (easter egg redacted)


## Tasks

- [x] remove template art work and sound effects
- [x] spawn the whale
- [x] show the controls (minimal tutorial)
- [x] spawn starting waves
- [x] periodically spawn other waves
- [x] waves move
- [x] background music
- [x] whale movement
  - [x] turn the whale when arrow keys are pressed
  - [x] move the waves diagonally when arrow keys pressed
- [ ] add encounters
  - [ ] birds
    - [x] bird graphics added to `creature.png`
    - [x] bird spawner
    - [ ] birds randomly spawn every 10-15 seconds
    - [ ] if birds get close, they circle the whale
    - [ ] after a while, the birds leave
    - [ ] bird sound effects
  - [ ] ships
  - [ ] fish
  - [ ] other whales
    - [ ] whalesong
    - [ ] forming a pod
- [ ] Weather cycles
  - [ ] day/night
  - [ ] wind
  - [ ] rain
  - [ ] storm
  - [ ] snow
- [ ] polish
  - [x] restyle the menu
  - [ ] update the credits
- [ ] stretch goals
  - [ ] periodically come up to breathe
  - [x] store whale position in a resource
  - [x] make rotate + wave movement lerp smoothly transition between left/off/right instead of jumping immediately
  - [ ] don't allow whale rotation until the whale has fully completed its entrance animation and is positioned on the screen
  - [ ] add the easter egg
  - [ ] add more encounters
  - [ ] add more music
  - [x] use something like `bevy_tween` to add more "juice" to whale entry animation
  - [ ] use something like `bevy_trickfilm` to make animations easier
- [ ] bugs
  - [ ] starting wave animations all start at the same frame so they're all in sync
  - [x] crash in movement when window minimised or in the background due to `windows.single()`
- [ ] jam admin
  - [x] update `Cargo.toml`
  - [x] fix CI yaml file
  - [ ] set up itch page
  - [ ] do a test release
  - [ ] do a "Final final v1 latest" release before the jam ends
  - [ ] deploy to itch
  - [ ] test deployed version


## Template

This repo used a slightly modified version of the [bevy quickstart template](https://github.com/TheBevyFlock/bevy_quickstart/).

## License

The source code in this repository is licensed under any of the following at your option:

- [MIT License](./LICENSE-MIT.txt)
- [Apache License, Version 2.0](./LICENSE-Apache-2.0.txt)

Assets are licensed under

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
