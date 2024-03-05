use gilrs::{Axis, Button, Gilrs, GilrsBuilder};
use integer_sqrt::IntegerSquareRoot;


pub fn calc_motors(inp: &[i32; 4]) -> [i32; 4] {
    // input array
    // lift, roll, pitch, yaw
    //      Z,      L,      M,      N
    //ae1 -0.25	    0	    0.5	    -0.25
    //ae2 -0.25	    -0.5	0	    0.25
    //ae3 -0.25	    0	    -0.5	-0.25
    //ae4 -0.25	    0.5	    0	    0.25

    let b = 1; // ????
    let d = 1; // ????

    let a1 = (-25 * inp[0] + 50 * inp[2]) / b - (25 * inp[3]) / d;
    let a2 = (-25 * inp[0] - 50 * inp[1]) / b + (25 * inp[3]) / d;
    let a3 = (-25 * inp[0] - 50 * inp[2]) / b - (25 * inp[3]) / d;
    let a4 = (-25 * inp[0] + 50 * inp[1]) / b + (25 * inp[3]) / d;

    // let mut ae1: i32 = 0;
    // let mut ae2: i32 = 0;
    // let mut ae3: i32 = 0;
    // let mut ae4: i32 = 0;
    
    // if a1 > 0 {
    //     ae1 = a1.integer_sqrt();
    // }
    // if a2 > 0 {
    //     ae2 = a2.integer_sqrt();
    // }
    // if a3 > 0 {
    //     ae3 = a3.integer_sqrt();
    // }
    // if a4 > 0 {
    //     ae4 = a4.integer_sqrt();
    // }

    // [u16::try_from(ae1).ok().unwrap(), u16::try_from(ae2).ok().unwrap(), u16::try_from(ae3).ok().unwrap(), u16::try_from(ae4).ok().unwrap()]
    [a1, a2, a3, a4]
}




#[derive(Debug)]
pub struct Joystick{
    Xaxis:f32,
    Yaxis:f32,
    abort:bool,
    pub throttle:f32,
    twist:f32,
}

pub struct DataBuff{
    // x:[f32],
    // y:[f32],
}


impl Joystick {
	pub fn to_yprt(&self) -> (i32, i32, i32, i32) {
		// Convert joystick input to yaw, pitch, and roll
		// The joystick input is in the range [-1.0, 1.0]
		// with max joystick roll and pitch are pi/4 radians
		// yaw is max 0.01 radians per sec
		let x = self.Xaxis;
		let y = self.Yaxis;
		let z = self.twist;
		let roll = (x * 785.0) as i32;
		let pitch = (y * 785.0) as i32;
		let yaw = (z * 10.0) as i32;
		let mut throttle = 0;
		if self.throttle > 0.01 {
			throttle = (self.throttle * 1000.0) as i32;
		}

		(yaw, pitch, roll, throttle)
	}
}


pub fn read_joystick(gilrs: &mut Gilrs) -> Option<Joystick> {
	// Process all events from the gilrs library
	while let Some(_event) = gilrs.next_event() {};

	// Initialize the joystick state
	let mut joystick = None;

	// Iterate over all connected gamepads
	for (_id, gamepad) in gilrs.gamepads() {
		// Update the joystick state based on gamepad input
		joystick = Some(Joystick {
			Xaxis: gamepad.value(Axis::LeftStickX),
			Yaxis: gamepad.value(Axis::LeftStickY),
			abort: gamepad.is_pressed(Button::South),
			throttle: ((gamepad.value(Axis::RightStickY) / 2.0) + 0.5) as f32,
			twist: gamepad.value(Axis::RightStickX),
		});
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
		match js {
			Some(js) => {
				println!("{:?}", js);
				let (y, p, r, t) = js.to_yprt();
				let motors = calc_motors(&[t, r, p, y]);
				println!("{:?}", motors);

			},
			None => println!("No joystick connected"),
		}
		std::thread::sleep(delay);
	}
}