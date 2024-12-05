#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ByteOrder {
    MsbFirst = 0x42,
    LsbFirst = 0x6C,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ConnectionStatus {
    Failed = 0,
    Authenticate = 2,
    Success = 1,
}

impl std::convert::From<u8> for ConnectionStatus {
    fn from(value: u8) -> Self {
        if value > 2 {
            panic!("invalid value for ConnectionStatus type",)
        } else if value == 0 {
            Self::Failed
        } else if value == 1 {
            Self::Success
        } else {
            Self::Authenticate
        }
    }
}

pub struct ConnectionInfo<'a> {
    pub order: ByteOrder,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub auth_protocol_name: &'a str,
    pub auth_protocol_data: &'a str,
}

#[derive(Debug)]
pub struct ConnectionErrorInfo {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub reason: String,
}

#[derive(Debug)]
pub struct ConnectionAuthenticateInfo {}

#[derive(Debug)]
pub struct ConnectionSuccessInfo {
    
}

#[derive(Debug)]
pub struct ConnectionAuthenticateOrSuccessInfo {
    pub success_info: Option<ConnectionSuccessInfo>,
    pub authenticate_info: Option<ConnectionAuthenticateInfo>,
    pub success: bool,
}
