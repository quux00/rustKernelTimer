#[allow(ctypes)];
#[no_std];
#[fixed_stack_segment];
//Wil Thomason, wbt9mh
mod zero;

extern {
	#[fast_ffi]
    pub fn signal(sig: i32, func: extern fn() -> i32) -> i32;
    #[fast_ffi]
    pub fn write(fd: i32, buf: *u8, nbyte: uint) -> uint;
}

#[abi="cdecl"]
extern {
	pub fn precise_time_ns(ns: &mut u64);
}

struct timeval {
	tv_sec : i64,
	tv_usec : i64
}

struct itimerval {
	it_interval : timeval,
	it_value : timeval
}

#[nolink]
#[abi="cdecl"]
extern {
	pub fn exit(status: i32) -> !;
	pub fn setitimer(which : i32, new : *itimerval, old : *itimerval) -> i32;
}

fn main() {
	unsafe { 
		signal(14 , timer_handler);
		let tim = itimerval { it_interval : timeval { tv_sec: 0,  tv_usec : 250}, it_value : timeval { tv_sec: 0,  tv_usec : 250}};
		let tim2 = itimerval { it_interval : timeval { tv_sec: 0,  tv_usec : 250}, it_value : timeval { tv_sec: 0,  tv_usec : 250}};
		setitimer(0, &tim, &tim2);
	}
	loop {
	}
}

fn doNothing() {

}

fn timer_handler() -> i32 {
	static mut counter : int = 0;
	static mut firstTime : u64 = 0;
	unsafe {
		//write(1, &"called\n"[0], 7);
	if counter == 0 {
		precise_time_ns(&mut firstTime);
	}
		counter += 1;
	

	if counter >= 10000 {
		setitimer(0, 0 as *itimerval, 0 as *itimerval);
		let mut newTime : u64 = 0;
		precise_time_ns(&mut newTime);
		let difference = newTime - firstTime;
		let delta = (difference as f64)/(counter as f64);
		let mut secs : f64 = (delta) / (1000000000 as f64);
		secs = (1.0/secs) ;
		write(1, &"Approximate kernel timer frequency is (in Hz): "[0], 47);
		let mut mults = 0;
		while secs > 1.0 {
			secs /= 10.0;
			mults += 1;
		}

		while mults > -3 {
			if mults == 0 {
				write(1, &"."[0], 1);
			}
			mults -= 1;
			secs *= 10.0;
			match (secs % 10.0) as int
			{
				0 => {write(1, &"0"[0], 1);}
				1 => {write(1, &"1"[0], 1);}
				2 => {write(1, &"2"[0], 1);}
				3 => {write(1, &"3"[0], 1);}
				4 => {write(1, &"4"[0], 1);}
				5 => {write(1, &"5"[0], 1);}
				6 => {write(1, &"6"[0], 1);}
				7 => {write(1, &"7"[0], 1);}
				8 => {write(1, &"8"[0], 1);}
				9 => {write(1, &"9"[0], 1);}
				_ => {}
			}
		}

		write(1, &"\n"[0], 1);
		exit(0)
		}
	}
	1
}