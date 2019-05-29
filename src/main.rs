
use rand::distributions::{Distribution, Normal};
use rand::Rng;
use std::ops::Not;
use std::time::Duration;
use std::thread;
use rppal::gpio::{Gpio, Error, OutputPin};

const OFF_TIME_MEAN : f64 = 15000.0;
const OFF_TIME_STD_DEV : f64 = 3000.0;
const ON_TIME_MEAN : f64 = 10000.0;
const ON_TIME_STD_DEV : f64 = 1000.0;
const START_ON : bool = true;
const OUTPUT_PIN : u8 = 4;

#[derive(PartialEq)]
enum State {
    On, Off
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

impl From<bool> for State {
    fn from(b : bool) -> Self {
        if b { State::On } else { State::Off }
    }
}

fn generate_time<R: Rng + Sized>(rng : &mut R, state : &State) -> f64 {
    match state {
        State::Off => Normal::new(OFF_TIME_MEAN, OFF_TIME_STD_DEV),
        State::On => Normal::new(ON_TIME_MEAN, ON_TIME_STD_DEV),
    }.sample(rng)
}

fn update_gpio(pin : &mut OutputPin, state : &State) {
    // update gpio pin to match state
    match state {
        State::On => {
            println!("On");
            pin.set_high();
        },
        State::Off => {
            println!("Off");
            pin.set_low();
        },
    };
}

fn connect_gpio() -> Result<OutputPin, Error> {
    Ok(Gpio::new()?.get(OUTPUT_PIN)?.into_output())
}

fn main() {
    let mut state : State = START_ON.into();
    let mut rng = rand::thread_rng();
    let mut pin = connect_gpio().expect("Pin not available.");
    loop {
        update_gpio(&mut pin, &state);
        let delay_ms = generate_time(&mut rng, &state).round();
        let delay_ms_clamped = if delay_ms > 0.0 { delay_ms } else { 0.0 };
        let dur = Duration::from_millis(delay_ms_clamped as u64);
        thread::sleep(dur);
        state = !state;
    }
}
