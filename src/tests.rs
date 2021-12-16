use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn create_server() {
    let term_signal = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::signal::SIGINT, term_signal.clone()).unwrap();

    let mut count = 0;

    use crate::*;

    let mut parameters = Parameters::default();
    parameters.drm_formats = vec![
        DrmFormat {
            code: DrmFourcc::Abgr8888,
            modifier: DrmModifier::Linear,
        },
        DrmFormat {
            code: DrmFourcc::Xrgb8888,
            modifier: DrmModifier::Linear,
        },
    ];

    let mut server = EmbeddedWaylandServer::new(parameters);
    server.create_seat(0, "Seat-0");
    server.add_keyboard(0, 200, 25);
    server.add_cursor(0);
    server.create_output(
        1,
        "Outpu1",
        PhysicalProperties {
            size: (1920, 1080).into(),
            subpixel: Subpixel::None,
            make: String::from(""),
            model: String::from(""),
        },
    );
    while !term_signal.load(Ordering::Relaxed) {
        let requests = server.dispatch(std::time::Duration::from_secs(1));
        if !requests.is_empty() {
            println!("Outside events {:#?}", requests);
        }

        requests.iter().for_each(|request| {
            match request {
                #[cfg(feature = "xdg_shell")]
                WaylandRequest::XdgRequest { request } => {
                    match request {
                        XdgRequest::NewToplevel { surface } => {
                            surface.send_configure();
                            //println!("{:#?}",surface.get_surface().unwrap().as_ref().client().unwrap());
                            with_states(&surface.get_surface().unwrap(), |surface_data| {
                                let client_id = ClientId(count);
                                count += 1;
                                surface_data.data_map.insert_if_missing(|| client_id);
                                println!(
                                    "Committing {:#?}: {:#?}",
                                    surface,
                                    surface_data.cached_state.pending::<SurfaceAttributes>()
                                );
                                println!(
                                    "Current: {:#?}",
                                    surface_data.cached_state.current::<SurfaceAttributes>()
                                );
                            })
                            .unwrap();
                        }
                        _ => (),
                    }
                }
                WaylandRequest::Commit { surface } => {
                    with_states(&surface, |surface_data| {
                        let surface_attributes =
                            surface_data.cached_state.current::<SurfaceAttributes>();
                        surface_attributes
                            .buffer
                            .as_ref()
                            .map(|buffer| match buffer {
                                BufferAssignment::NewBuffer { buffer, delta } => {
                                    let dma_buf = buffer.as_ref().user_data().get::<Dmabuf>();
                                    println!("Dmabuf: {:#?}", dma_buf);
                                }
                                _ => (),
                            });
                    })
                    .unwrap();
                }
                _ => {}
            }
        })
    }
}
