use crate::types::{self, ByteOrder};
use byteorder::{BigEndian, ByteOrder as ExternalByteOrder, LittleEndian};
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

pub fn send_u8_to_stream(
    stream: &mut UnixStream,
    data: u8,
    order: ByteOrder,
) -> Result<usize, std::io::Error> {
    match order {
        ByteOrder::MsbFirst => send_to_stream(stream, &data.to_le_bytes()),
        ByteOrder::LsbFirst => send_to_stream(stream, &data.to_be_bytes()),
    }
}

pub fn send_u16_to_stream(
    stream: &mut UnixStream,
    data: u16,
    order: ByteOrder,
) -> Result<usize, std::io::Error> {
    match order {
        ByteOrder::MsbFirst => send_to_stream(stream, &data.to_le_bytes()),
        ByteOrder::LsbFirst => send_to_stream(stream, &data.to_be_bytes()),
    }
}

pub fn send_u32_to_stream(
    stream: &mut UnixStream,
    data: u32,
    order: ByteOrder,
) -> Result<usize, std::io::Error> {
    match order {
        ByteOrder::MsbFirst => send_to_stream(stream, &data.to_le_bytes()),
        ByteOrder::LsbFirst => send_to_stream(stream, &data.to_be_bytes()),
    }
}

pub fn skip_write_bytes(stream: &mut UnixStream, num: usize) -> Result<usize, std::io::Error> {
    send_to_stream(stream, &[0].repeat(num))
}

pub fn send_to_stream(stream: &mut UnixStream, data: &[u8]) -> Result<usize, std::io::Error> {
    stream.write(data)
}

pub fn read_u8_from_stream(
    stream: &mut UnixStream,
    order: ByteOrder,
) -> Result<u8, std::io::Error> {
    let _ = order;
    let data = read_from_stream(stream, 1);
    if let Ok(data) = data {
        Ok(data[0])
    } else {
        Err(data.unwrap_err())
    }
}

pub fn read_u16_from_stream(
    stream: &mut UnixStream,
    order: ByteOrder,
) -> Result<u16, std::io::Error> {
    let data = read_from_stream(stream, 2);
    if let Ok(mut data) = data {
        match order {
            ByteOrder::LsbFirst => Ok(BigEndian::read_u16(data.as_mut_slice())),
            ByteOrder::MsbFirst => Ok(LittleEndian::read_u16(data.as_mut_slice())),
        }
    } else {
        Err(data.unwrap_err())
    }
}

pub fn read_u32_from_stream(
    stream: &mut UnixStream,
    order: ByteOrder,
) -> Result<u32, std::io::Error> {
    let data = read_from_stream(stream, 4);
    if let Ok(mut data) = data {
        match order {
            ByteOrder::LsbFirst => Ok(BigEndian::read_u32(data.as_mut_slice())),
            ByteOrder::MsbFirst => Ok(LittleEndian::read_u32(data.as_mut_slice())),
        }
    } else {
        Err(data.unwrap_err())
    }
}

pub fn read_from_stream(stream: &mut UnixStream, num: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes = [0 as u8].repeat(num);
    let read_result = stream.read(bytes.as_mut_slice());
    if read_result.is_err() {
        Err(read_result.unwrap_err())
    } else {
        Ok(bytes)
    }
}

pub fn skip_read_bytes(stream: &mut UnixStream, num: usize) -> Result<(), std::io::Error> {
    let val = read_from_stream(stream, num);
    if val.is_err() {
        Err(val.unwrap_err())
    } else {
        Ok(())
    }
}

pub fn pad(val: usize) -> usize {
    (4 - (val % 4)) % 4
}
