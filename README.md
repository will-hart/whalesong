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
- [x] encounter spawning system
- [x] whale movement
  - [x] turn the whale when arrow keys are pressed
  - [x] move the waves diagonally when arrow keys pressed
- [x] add encounters
  - [x] birds
    - [x] bird graphics added to `creature.png`
    - [x] bird spawner
    - [x] birds randomly spawn every X seconds
    - [x] if birds get close, they circle the whale
    - [x] after a while, the birds leave
    - [x] add a "wander in area" behaviour while following the whale
    - [x] bird sound effects
  - [x] schools of fish
    - [x] create fish sprite
    - [x] spawn fish
    - [x] fish boids
    - [x] fish encounters
    - [x] fish avoid the whale
  - [x] ships
    - [x] ship sprites
    - [x] ship wake
    - [x] engine noise
    - [x] ship occludes waves, fish, whale but not birds
  - [x] other whales
    - [x] whalesong
- [x] Weather cycles
  - [x] day/night
  - [x] rain
  - [x] snow
  - [x] icebergs
- [x] polish
  - [x] restyle the menu
  - [x] make rotate + wave movement lerp smoothly transition between left/off/right instead of jumping immediately
  - [x] when the whale turns there is a lot of empty space at the screen edges - no waves
  - [x] move whale not things around the whale
  - [x] update the credits
  - [x] refactor whale movement to make it a bit less janky when turning
  - [x] refactor whale following movement
  - [x] fix baby not displaying
  - [x] fix baby whale movement
  - [x] fix flipped movement
  - [x] refactor spawning rates to correspond with distance a bit better
  - [x] snow first when travelling north
  - [x] fix flip text
  - [x] ship horn only sounds once
- [x] code "quality" lol
  - [x] store whale position in a resource
- [x] stretch goals
  - [x] periodically come up to breathe
  - [x] don't allow whale rotation until the whale has fully completed its entrance animation and is positioned on the screen
  - [x] Display player help icons after movement complete.
  - [x] add the movement easter egg (its a secret)
  - [x] add the cycles easter egg (its also a secret)
    - [x] flip camera
    - [x] flip anything that needs flipping
    - [x] flip the controls
    - [x] flip on a timer
    - [x] flip text
    - [x] add phase 1 on the northern flip 
      - [x] make it 100% likely on flip 1
      - [x] make it not 100% on future flips
    - [x] add phase 2 on the southern flip (if the northern one occurs)
- [x] bugs
  - [x] starting wave animations all start at the same frame so they're all in sync
  - [x] crash in movement when window minimised or in the background due to `windows.single()`
  - [x] move targets aren't updated on screen resize, meaning that things just pile up on the screen edges
- [x] jam admin
  - [x] update `Cargo.toml`
  - [x] fix CI yaml file
  - [x] set up itch page
  - [x] do a test release
  - [x] work out why the itch build isn't working
  - [x] do a "Final final v1 latest" release before the jam ends
  - [x] deploy to itch
  - [x] add screenshots and details to the jam page
  - [x] test deployed version

These ideas were abandoned due to it being a game jam game:

  - weather
    - wind
    - storms
  - stretch goals
    - add more encounters
    - add islands
    - add more music variety
    - ships avoid icebergs :grimace:
    - whale turning animation
  - ship volume depends on proximity to the whale
  - icebergs have different sizes without blurring

## Credits

- I've included https://github.com/DanielDK05/bevoids/ in this repo with the following changes:
  - update for bevy 0.14.0
  - allow setting velocity directly
  - add a `BoidJitter` component which adds a bit of random jitter to the movement
  - add a `BoidRepulsor` component which isn't a boid but pushes boids away
- Sounds with "complex" names were taken from freesounds.org, and were all CC0 licensed.

## Template

This repo used a slightly modified version of the [bevy quickstart template](https://github.com/TheBevyFlock/bevy_quickstart/).

## License

The source code in this repository is licensed under any of the following at your option:

- [MIT License](./LICENSE-MIT.txt)
- [Apache License, Version 2.0](./LICENSE-Apache-2.0.txt)

Assets are licensed under

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
