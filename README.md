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

## Task tracking

See [./tasks.todo](./tasks.todo)

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
