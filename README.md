# Embeddable wayland server
This is on ongoing development, so expect breaking changes.

An embeddable wayland server with the objective to simplify for compositor the management of the Wayland logic.
The main function of the library is the `EmbeddableWaylandServer::dispatch` function. It will return a list of `WaylandRequest`, a big enum that contains all the possible requests.
The user can then iterate over that list to manage and reply to the client's requests. 
Multiple instantiated globals, like `WlSeat` and `WlOutput`, will be internally managed and make them accessibile thought add,get and remove functions.
This library just help to manage the Wayland logic, does not contain the compositor logic.
To satisfy the Wayland protocols the compositor need to handle the requests and reply to them accordingly.

This library rely on the Smithay projects to work and their structures are returned as part of the events, so the user is required to know how to handle them:
- https://smithay.github.io/wayland-rs/wayland_server/
- https://smithay.github.io/wayland-rs/wayland_protocols/

The Smithay developer work allow to Rust to enter in the Wayland world with a simple and clean interface,
so if you like it, consider contributing to them to improve the ecosystem.

Based on the enabled features, user can select which part of the Wayland protocol he is going to support:
- `xdg_shell`
