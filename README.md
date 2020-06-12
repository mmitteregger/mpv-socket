mpv-socket
==========

A Rust library for the JSON-based IPC protocol of mpv.

It currently supports Windows only, but this limitation will be lifted in the future.

## Example

Here's a simple example.

Please note that it requires a running mpv player that is started with the 
`--input-ipc-server=\\.\pipe\mpv-socket` option, otherwise `MpvSocket::connect(...)` will fail. 

```rust
use mpv_socket::{Error, MpvSocket, Property};

fn main() -> Result<(), Error> {
    pretty_env_logger::init_timed();

    let mut mpv_socket = MpvSocket::connect(r#"\\.\pipe\mpv-socket"#)?;

    let client_name = mpv_socket.client_name()?;
    let version = mpv_socket.get_version()?;
    let filename: String = mpv_socket.get_property(Property::Filename)?;

    println!("Client name: {}", client_name);
    println!("Version: {}", version);
    println!("Filename: {}", filename);

    // Observe property changes with a iterator based API:
    for result in mpv_socket
        .observe_property(Property::PlaybackTime)?
        .take(10)
    {
        let playback_time: f64 = result?;
        log::info!("Playback time: {}", playback_time);
    }

    Ok(())
}
```

Check out the [examples directory](./examples) for more.

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
