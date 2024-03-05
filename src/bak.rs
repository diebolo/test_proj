use gilrs::{Gilrs, Axis, Button};

// Define a struct to represent the joystick state
#[derive(Debug)]
pub struct Joystick {
	Xaxis: f32,
	Yaxis: f32,
	abort: bool,
	throttle: f32,
	twist: f32,
}

// Function to read joystick input and return the joystick state
pub fn read_joystick(gilrs: &mut Gilrs) -> Joystick {
	// Process all events from the gilrs library
	while let Some(event) = gilrs.next_event() {
		// Handle connected events
		match event.event {
			gilrs::EventType::Connected => {
				println!("Gamepad connected: {:?}", event.id);
			}
			gilrs::EventType::Disconnected => {
				println!("Gamepad disconnected: {:?}", event.id);
			}
			_ => {}
		}
	}

	// Initialize the joystick state
	let mut joystick = Joystick {
		Xaxis: 0.0,
		Yaxis: 0.0,
		abort: false,
		throttle: 0.0,
		twist: 0.0,
	};

	// Iterate over all connected gamepads
	for (_id, gamepad) in gilrs.gamepads() {
		// Update the joystick state based on gamepad input
		//println!("{:?}", gamepad.state());
		joystick = Joystick {
			Xaxis: gamepad.value(Axis::LeftStickX),
			Yaxis: gamepad.value(Axis::LeftStickY),
			abort: gamepad.is_pressed(Button::South),
			throttle: gamepad.value(Axis::RightStickY) as f32 / 2.0 + 0.5,
			twist: gamepad.value(Axis::RightStickX),
		};
	}

	// Return the joystick state
	return joystick;
}

fn main() {
	// Create a new instance of the Gilrs library
	let mut gilrs = GilrsBuilder::new()
		.add_included_mappings(true)
		.add_env_mappings(false)
		.build().unwrap();
	let delay = std::time::Duration::from_millis(100);

	// gilrs.set_mapping(gamepad_id, mapping, name)

	// Continuously read joystick input and print the joystick state
	loop {
		let js = read_joystick(&mut gilrs);
		println!("{:?}", js);
		std::thread::sleep(delay);
	}
}


use gilrs::{Gilrs, Button, Event};
fn main() {
	let mut gilrs = GilrsBuilder::new()
		.add_included_mappings(true)
		.add_env_mappings(true)
		.add_mappings(&sdl_string)
		.build().unwrap();

	// Iterate over all connected gamepads
	for (_id, gamepad) in gilrs.gamepads() {
		println!("{} is {:?}", gamepad.name(), gamepad.power_info());
	}

	let mut active_gamepad = None;

	loop {
		// Examine new events
		while let Some(Event { id, event, time }) = gilrs.next_event() {
			println!("{:?} New event from {}: {:?}", time, id, event);
			active_gamepad = Some(id);
		}

		// You can also use cached gamepad state
		if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
			if gamepad.is_pressed(Button::South) {
				println!("Button South is pressed (XBox - A, PS - X)");
			}
		}
	}
}


use gilrs::{Button, Event, GilrsBuilder};
fn main() {
	let mut gilrs = GilrsBuilder::new()
		.add_included_mappings(true)
		.add_env_mappings(true)
		//.add_mappings(&sdl_string)
		.build().unwrap();

	// Iterate over all connected gamepads
	for (_id, gamepad) in gilrs.gamepads() {
		println!("{} is {:?}", gamepad.name(), gamepad.power_info());
	}

	let mut active_gamepad = None;

	loop {
		// Examine new events
		while let Some(Event { id, event, time }) = gilrs.next_event() {
			println!("{:?} New event from {}: {:?}", time, id, event);
			active_gamepad = Some(id);
		}

		// You can also use cached gamepad state
		if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
			if gamepad.is_pressed(Button::South) {
				println!("Button South is pressed (XBox - A, PS - X)");
			}
		}
	}
}