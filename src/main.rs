use gilrs::{Gilrs, Axis, Button, GilrsBuilder};

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
		.add_included_mappings(false)
		.add_env_mappings(false)
		.add_mappings("030000006d04000015c2000010010000,Logitech Extreme 3D,a:b0,leftx:a0,lefty:a1,rightx:a2,righty:a3,platform:Linux")
		.build().unwrap();	
	let delay = std::time::Duration::from_millis(100);


	// Continuously read joystick input and print the joystick state
	loop {
		let js = read_joystick(&mut gilrs);
		println!("{:?}", js);
		std::thread::sleep(delay);
	}
}