use mpv_socket::event::PropertyChangeEvent;
use mpv_socket::{Error, MpvSocket, Property};

fn main() -> Result<(), Error> {
    pretty_env_logger::init_timed();

    let mut mpv_socket = MpvSocket::connect(r#"\\.\pipe\mpv-socket"#)?;

    // Observe multiple properties:
    for result in mpv_socket
        .observe_properties(
            [
                Property::Filename,
                Property::Seeking,
                Property::Pause,
                Property::Volume,
                Property::PercentPos,
                Property::PlaybackTime,
            ]
            .iter()
            .copied(),
        )?
        .take(10)
    {
        let event: PropertyChangeEvent = result?;
        log::info!("Property \"{}\" changed to: {}", event.name, event.data);
    }

    Ok(())
}
