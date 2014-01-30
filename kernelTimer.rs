// Wil Thomason
// Modified by Michael Peterson
extern mod extra;

use extra::time;
use std::libc;

static SIGALRM_INTERVAL_IN_USECS: i64 = 250;
static LOOPS: int = 5000;

extern {
    pub fn signal(sig: i32, func: extern fn() -> i32) -> i32;
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
// #[abi="cdecl"] => needs to change to:  `extern "abi" fn`  ==> how?
extern {
    pub fn setitimer(which : i32, new : *itimerval, old : *itimerval) -> i32;
}

fn main() {
    unsafe { 
        signal(14, timer_handler);
        let tim  = itimerval { it_interval : timeval { tv_sec: 0,  tv_usec : SIGALRM_INTERVAL_IN_USECS},
                               it_value    : timeval { tv_sec: 0,  tv_usec : SIGALRM_INTERVAL_IN_USECS} };
        let tim2 = itimerval { it_interval : timeval { tv_sec: 0,  tv_usec : SIGALRM_INTERVAL_IN_USECS},
                               it_value    : timeval { tv_sec: 0,  tv_usec : SIGALRM_INTERVAL_IN_USECS} };
        setitimer(0, &tim, &tim2);
    }
    loop {
    }
}

fn timer_handler() -> i32 {
    static mut counter : int = 0;
    static mut firstTime : u64 = 0;
    unsafe {
        if counter == 0 {
            firstTime = time::precise_time_ns();
        }
        counter += 1;
        

        if counter >= LOOPS {
            setitimer(0, 0 as *itimerval, 0 as *itimerval);
            let newTime : u64 = time::precise_time_ns();
            let difference = newTime - firstTime;
            let delta = (difference as f64)/(counter as f64);
            let mut secs : f64 = (delta) / (1000000000 as f64);
            secs = (1.0/secs) ;
            let s = "Approximate kernel timer frequency is (in Hz): ".to_c_str();
            s.with_ref(|x| libc::puts(x));
            let f = secs.to_str().to_c_str();
            f.with_ref(|x| libc::puts(x));
            let n = "\n".to_c_str();
            n.with_ref(|x| libc::puts(x));
            libc::exit(0)
        }
        1
    }
}
