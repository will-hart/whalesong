# Bevoids
A **WIP** bevy plugin that aims to make boids easy to add to your game.

[![Crates.io](https://img.shields.io/crates/v/bevoids)](https://crates.io/crates/bevoids)

![Gif showing 2d boids](gifs/2d_boids.gif)

# Bevy 0.14?
I am currently working (on and off) on an update to bevy 0.14, but I might have fallen down a rabbit hole of making the plugin perform better in the same branch (like you do).. Post an issue if it takes too long! 

## How to use
I recommend to check out [examples](examples), to see how bevoids is used.

### Adding the plugin, and configuring
For the plugin to work, you need to register it.

You also need to add the BoidsConfig resource, where you configure the space (2d or 3d), and enable/disable debugging:

```rust
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoidsPlugin))
        .insert_resource(BoidsConfig {
            space: BoidSpace::TwoDimensional,
            debug: false,
        })
        .run();
}
```

### Creating a Boid entity.
To create a boid entity, add the Boid component to your entity:

```rust
commands.spawn((
      MaterialMesh2dBundle {
          mesh: meshes
              .add(shape::RegularPolygon::new(10.0, 3).into())
              .into(),
          material: materials.add(ColorMaterial::from(Color::WHITE)),
          transform: Transform::from_xyz(x as f32 * spacer, y as f32 * spacer, 0.0),
          ..default()
      },
      Boid::new(
          BoidSpeed::new(BOID_MIN_SPEED, BOID_MAX_SPEED),
          BoidTurningStrength::new(
              BOID_COHESION,
              BOID_SEPARATION,
              BOID_ALIGNMENT,
              BOID_BORDER_TURN_STRENGTH,
          ),
          BoidViewConfig::new(BOID_FOV, BOID_PROTECTED_RANGE, BOID_VIEW_RANGE),
      )
  ));
}
```

### Limiting the Boid's area of movement
If you want the boid to only move within a restricted area, add the BoidBorder component to it.

#### The component works like this:
It has 6 parameters; left, right, top, etc... Each of these parameters is an `Option<(f32, f32)>`, 
where the first `f32` represents the position of the border, and the second represents a margin for when the boid should start turning.

Each side is optional, to give you the choice of where to restrict.
You can for example only restrict the top and bottom, for birds that can roam the entire world!

```rust
commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(10.0, 3).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(x as f32 * spacer, y as f32 * spacer, 0.0),
            ..default()
        },
        Boid::new(
            BoidSpeed::new(BOID_MIN_SPEED, BOID_MAX_SPEED),
            BoidTurningStrength::new(
                BOID_COHESION,
                BOID_SEPARATION,
                BOID_ALIGNMENT,
                BOID_BORDER_TURN_STRENGTH,
            ),
            BoidViewConfig::new(BOID_FOV, BOID_PROTECTED_RANGE, BOID_VIEW_RANGE),
        ),
        BoidBorder {
            left: Some((-BORDER_WIDTH / 2.0, BORDER_MARGIN)),
            right: Some((BORDER_WIDTH / 2.0, BORDER_MARGIN)),
            top: Some((BORDER_HEIGHT / 2.0, BORDER_MARGIN)),
            bottom: Some((-BORDER_HEIGHT / 2.0, BORDER_MARGIN)),
            front: None,
            back: None,
        },
    ));
}
```

## Examples

| Example name | Scene                             |
|--------------|-----------------------------------|
| simple2d     | A simple 2d scene with 400 boids  |
| simple3d     | A simple 3d scene with 1000 boids |

To run example, run `cargo run --example <example-name>`

## License
This plugin is licensed under the MIT license:

Copyright (c) 2024 Daniel Kalstad

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Credits

I made this plugin completely alone, but here are some sites I used for reference:
- https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html
- https://vergenet.net/~conrad/boids/pseudocode.html
