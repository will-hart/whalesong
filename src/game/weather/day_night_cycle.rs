use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

// The amount of world time that elapses per game second
const TIME_OF_DAY_HOURS_PER_GAME_SECONDS: f32 = 1.5;
const NUM_COLOURS: usize = 8;
const HOURS_PER_COLOUR: f32 = 24.0 / (NUM_COLOURS as f32);
const CHANCE_OF_SUN: f64 = 0.8;

// Note for smooth lerping, these palettes should start and end on the same colour as each other
const SUNNY_COLOR_CYCLE: [Vec3; NUM_COLOURS] = [
    /*  0am */ Vec3::new(0.97, 0.97, 0.85),
    /*  3am */ Vec3::new(0.97, 0.97, 0.85),
    /*  6am */ Vec3::new(0.97, 0.97, 0.85),
    /*  9am */ Vec3::new(0.97, 0.97, 0.85),
    /* 12pm */ Vec3::new(0.97, 0.97, 0.85),
    /*  3pm */ Vec3::new(0.97, 0.97, 0.85),
    /*  6pm */ Vec3::new(0.97, 0.97, 0.85),
    /*  9pm */ Vec3::new(0.97, 0.97, 0.85),
];

const STORMY_COLOR_CYCLE: [Vec3; NUM_COLOURS] = [
    /*  0am */ Vec3::new(0.97, 0.97, 0.85),
    /*  3am */ Vec3::new(0.97, 0.97, 0.85),
    /*  6am */ Vec3::new(0.97, 0.97, 0.85),
    /*  9am */ Vec3::new(0.97, 0.97, 0.85),
    /* 12pm */ Vec3::new(0.97, 0.97, 0.85),
    /*  3pm */ Vec3::new(0.97, 0.97, 0.85),
    /*  6pm */ Vec3::new(0.97, 0.97, 0.85),
    /*  9pm */ Vec3::new(0.97, 0.97, 0.85),
];

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
            time_of_day: 6.,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WeatherState>()
        .add_systems(Update, day_night_cycle.run_if(in_state(Screen::Playing)));
}

fn day_night_cycle(
    time: Res<Time>,
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
    clear_colour.0 = Color::srgb(colour.x, colour.y, colour.z);
}
