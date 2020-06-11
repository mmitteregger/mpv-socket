use mpv_socket::{Error, MpvSocket, Property};

fn main() -> Result<(), Error> {
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
        println!("Playback time: {}", playback_time);
    }

    Ok(())
}
