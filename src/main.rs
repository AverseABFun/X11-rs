mod connect;
mod types;
mod util;

use std::os::unix::net::UnixStream;

use connect::connect_to_server;
use types::ConnectionInfo;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/.X11-unix/X0")?;
    let info = ConnectionInfo {
        order: types::ByteOrder::LsbFirst,
        protocol_major_version: 2816,
        protocol_minor_version: 0,
        auth_protocol_name: "",
        auth_protocol_data: "",
    };
    println!(
        "{:#?}",
        connect_to_server(info, &mut stream).expect_err("got non-error!")
    );
    Ok(())
}
