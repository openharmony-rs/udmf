use udmf_sys::Udmf_ErrCode;

#[derive(Debug, thiserror::Error)]
pub enum UdmfError {
    #[error("Internal error: {0}")]
    InternalError(u32),
    #[error("Invalid parameter")]
    InvalidParam,
    #[error("Unknown error code: {0}")]
    Unknown(u32),
}

pub type Result<T> = std::result::Result<T, UdmfError>;

impl From<i32> for UdmfError {
    fn from(code: i32) -> Self {
        Self::from(code as u32)
    }
}

impl From<u32> for UdmfError {
    fn from(code: u32) -> Self {
        match Udmf_ErrCode(code) {
            Udmf_ErrCode::E_OK => panic!("E_OK is not an error"),
            Udmf_ErrCode::ERR => UdmfError::InternalError(code),
            Udmf_ErrCode::E_INVALID_PARAM => UdmfError::InvalidParam,
            _ => UdmfError::Unknown(code),
        }
    }
}

pub(crate) fn to_result(code: i32) -> Result<()> {
    if code == Udmf_ErrCode::E_OK.0 as i32 {
        Ok(())
    } else {
        Err(UdmfError::from(code))
    }
}
