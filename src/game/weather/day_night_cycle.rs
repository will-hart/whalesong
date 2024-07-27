//! The day / night cycle for the background colours. Borrows day night cycle stuff
//! from my Bevy Jam 2 entry, here
//! https://github.com/will-hart/bevy_jam_2/blob/main/src/game/day_night_cycle.rs

use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

// The amount of world time that elapses per game second
const TIME_OF_DAY_HOURS_PER_GAME_SECONDS: f32 = 0.4;
const NUM_COLOURS: usize = 8;
const HOURS_PER_COLOUR: f32 = 24.0 / (NUM_COLOURS as f32);
const CHANCE_OF_SUN: f64 = 0.8;

// just a helper to make it easier to transpose between affinity and bevy colours
macro_rules! p {
    ($val: expr) => {
        $val as f32 / 255.
    };
}

// Note for smooth lerping, these palettes should start and end on the same colour as each other
const SUNNY_COLOR_CYCLE: [Vec3; NUM_COLOURS] = [
    /*  0am */ Vec3::new(p!(189), p!(189), p!(199)),
    /*  3am */ Vec3::new(p!(206), p!(205), p!(196)),
    /*  6am */ Vec3::new(p!(230), p!(230), p!(210)),
    /*  9am */ Vec3::new(p!(247), p!(247), p!(219)),
    /*  9am */ Vec3::new(p!(249), p!(249), p!(216)),
    /*  3pm */ Vec3::new(p!(247), p!(240), p!(234)),
    /*  6pm */ Vec3::new(p!(219), p!(210), p!(215)),
    /*  9pm */ Vec3::new(p!(189), p!(189), p!(208)),
];

const STORMY_COLOR_CYCLE: [Vec3; NUM_COLOURS] = [
    /*  0am */ Vec3::new(p!(189), p!(189), p!(199)),
    /*  3am */ Vec3::new(p!(206), p!(205), p!(196)),
    /*  6am */ Vec3::new(p!(230), p!(230), p!(210)),
    /*  9am */ Vec3::new(p!(247), p!(247), p!(219)),
    /*  9am */ Vec3::new(p!(249), p!(249), p!(216)),
    /*  3pm */ Vec3::new(p!(247), p!(240), p!(234)),
    /*  6pm */ Vec3::new(p!(219), p!(210), p!(215)),
    /*  9pm */ Vec3::new(p!(189), p!(189), p!(208)),
];

pub const INITIAL_TIME_OF_DAY: f32 = 6.;

#[derive(Resource)]
pub struct WeatherState {
    pub sunny: [Vec3; NUM_COLOURS],
    pub stormy: [Vec3; NUM_COLOURS],
    pub is_sunny: bool,
    pub time_of_day: f32,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            sunny: SUNNY_COLOR_CYCLE,
            stormy: STORMY_COLOR_CYCLE,
            is_sunny: true,
            time_of_day: INITIAL_TIME_OF_DAY,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WeatherState>()
        .insert_resource(DayNightColour(Color::srgb(
            SUNNY_COLOR_CYCLE[3].x,
            SUNNY_COLOR_CYCLE[3].y,
            SUNNY_COLOR_CYCLE[3].z,
        )))
        .add_systems(
            Update,
            (day_night_cycle, tint_with_day_night_cycle)
                .chain()
                .run_if(in_state(Screen::Playing)),
        );
}

#[derive(Resource)]
pub struct DayNightColour(Color);

fn day_night_cycle(
    time: Res<Time>,
    mut dnc: ResMut<DayNightColour>,
    mut weather: ResMut<WeatherState>,
    mut clear_colour: ResMut<ClearColor>,
) {
    let dt = time.delta_seconds();
    let elapsed = dt * TIME_OF_DAY_HOURS_PER_GAME_SECONDS;
    let mut rng = rand::thread_rng();

    let prev_time_of_day = weather.time_of_day;
    weather.time_of_day = (weather.time_of_day + elapsed) % 24.0;

    // check if we've wrapped over midnight
    if prev_time_of_day > 23.0 && weather.time_of_day < 1.0 {
        // true if we've just wrapped day, we need to toggle the colour pattern
        weather.is_sunny = rng.gen_bool(CHANCE_OF_SUN);
    }

    let from_idx = (weather.time_of_day / HOURS_PER_COLOUR).floor() as usize;
    let to_idx = (from_idx + 1) % NUM_COLOURS;

    let cycle_data = if weather.is_sunny {
        weather.sunny
    } else {
        weather.stormy
    };

    let colour = cycle_data[from_idx].lerp(
        cycle_data[to_idx],
        (weather.time_of_day % HOURS_PER_COLOUR) / HOURS_PER_COLOUR,
    );
    dnc.0 = Color::srgb(colour.x, colour.y, colour.z);
    clear_colour.0 = dnc.0;
}

/// A marker component which notes that the marked sprite should be tinted with the day / night cycle.
/// used for e.g. ship background
#[derive(Component)]
pub struct TintWithDayNightCycle;

pub fn tint_with_day_night_cycle(
    dnc: Res<DayNightColour>,
    mut tinters: Query<&mut Sprite, With<TintWithDayNightCycle>>,
) {
    for mut tint in &mut tinters {
        tint.color = dnc.0;
    }
}
