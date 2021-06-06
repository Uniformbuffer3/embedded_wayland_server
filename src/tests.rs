extern fn handle_sigint(_signal: libc::c_int) {
    panic!("SIGINT detected, terminating!");
}

use std::sync::{Arc,atomic::{Ordering,AtomicBool}};

#[test]
fn test_server(){
    let term_signal = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::signal::SIGINT, term_signal.clone()).unwrap();

    use crate::WaylandManager;
    let mut wayland_manager = WaylandManager::new();

    while !term_signal.load(Ordering::Relaxed) {
        println!("{:#?}",wayland_manager.dispatch(std::time::Duration::from_secs(1)));
    }

}
