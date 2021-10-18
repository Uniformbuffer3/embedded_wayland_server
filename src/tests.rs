use std::sync::{atomic::{Ordering,AtomicBool}};
use crate::Request::XdgRequest;
#[test]
fn create_server(){
    let term_signal = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::signal::SIGINT, term_signal.clone()).unwrap();

    use crate::*;

    let mut server = EmbeddedWaylandServer::new(Parameters::default());
    server.create_seat("Seat-0");
    server.add_keyboard("Seat-0", 200, 25);
    server.add_cursor("Seat-0");
    server.create_output("Outpu1", PhysicalProperties{
        size: (1920,1080).into(),
        subpixel: Subpixel::None,
        make: String::from(""),
        model: String::from("")
    });
    while !term_signal.load(Ordering::Relaxed) {
        let requests = server.dispatch(std::time::Duration::from_secs(1));
        //if !requests.is_empty() {println!("Outside events {:#?}",requests);}

        requests.iter().for_each(|request|{
            match request{
                #[cfg(feature="xdg_shell")]
                XdgRequest(surface_request)=>{
                    match surface_request {
                        XdgRequest::NewToplevel{surface}=>{
                            surface.send_configure();
                        }
                        _=>()
                    }
                }
                Request::Commit(surface)=>{
                    with_states(&surface,|surface_data|{
                        let pending = surface_data.cached_state.pending::<SurfaceAttributes>();
                        println!("Committing {:#?}: {:#?}",surface,pending);

                        if surface_data.cached_state.has::<SurfaceAttributes>() {
                            println!("Current: {:#?}",surface_data.cached_state.current::<SurfaceAttributes>());
                        }
                    }).unwrap();
                }
                _=>{}
            }

        })
    }

}
