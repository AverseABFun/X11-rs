use std::{os::unix::net::UnixStream, str::from_utf8};

use crate::{
    types::{
        ConnectionAuthenticateOrSuccessInfo, ConnectionErrorInfo, ConnectionInfo, ConnectionStatus,
    },
    util::{
        pad, read_from_stream, read_u16_from_stream, read_u8_from_stream, send_to_stream,
        send_u16_to_stream, send_u8_to_stream, skip_read_bytes, skip_write_bytes,
    },
};

pub fn connect_to_server(
    info: ConnectionInfo,
    stream: &mut UnixStream,
) -> Result<ConnectionAuthenticateOrSuccessInfo, ConnectionErrorInfo> {
    send_u8_to_stream(stream, info.order as u8, info.order)
        .expect("unable to send byte order to stream");

    skip_write_bytes(stream, 1).expect("unable to skip byte");

    send_u16_to_stream(stream, info.protocol_major_version, info.order)
        .expect("unable to send version to stream");
    send_u16_to_stream(stream, info.protocol_minor_version, info.order)
        .expect("unable to send version to stream");

    send_u16_to_stream(
        stream,
        info.auth_protocol_name.as_bytes().len() as u16,
        info.order,
    )
    .expect("unable to send authorization protocol name length to stream");
    send_u16_to_stream(
        stream,
        info.auth_protocol_data.as_bytes().len() as u16,
        info.order,
    )
    .expect("unable to send authorization protocol data length to stream");

    skip_write_bytes(stream, 2).expect("unable to skip bytes");

    send_to_stream(stream, info.auth_protocol_name.as_bytes())
        .expect("unable to send authorization protocol name to stream");
    skip_write_bytes(stream, pad(info.auth_protocol_name.as_bytes().len()))
        .expect("unable to skip bytes");

    send_to_stream(stream, info.auth_protocol_data.as_bytes())
        .expect("unable to send authorization protocol data to stream");
    skip_write_bytes(stream, pad(info.auth_protocol_data.as_bytes().len()))
        .expect("unable to skip bytes");

    let status: ConnectionStatus = read_u8_from_stream(stream, info.order)
        .expect("unable to read status")
        .into();
    match status {
        ConnectionStatus::Failed => {
            let reason_length =
                read_u8_from_stream(stream, info.order).expect("unable to read reason length");

            let protocol_major_version =
                read_u16_from_stream(stream, info.order).expect("unable to read protocol version");
            let protocol_minor_version =
                read_u16_from_stream(stream, info.order).expect("unable to read protocol version");

            skip_read_bytes(stream, 2).expect("unable to skip bytes");

            let reason =
                read_from_stream(stream, reason_length as usize).expect("unable to read reason");

            skip_read_bytes(stream, pad(reason_length as usize)).expect("unable to skip bytes");

            Err(ConnectionErrorInfo {
                protocol_major_version,
                protocol_minor_version,
                reason: match from_utf8(&reason) {
                    Ok(v) => v.to_string(),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                },
            })
        }
        ConnectionStatus::Authenticate => {
            panic!("Not implemented!")
        }
        ConnectionStatus::Success => {
            panic!("Not implemented!")
        }
    }
}
