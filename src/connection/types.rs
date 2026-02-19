use super::*;
use crate::WlProxy;

#[derive(Debug)]
pub enum BindError {
    UnsupportedVersion,
    NotFound,
}

pub enum WlError {
    Io(std::io::Error),
    Protocol {
        proxy: WlProxy,
        code: i32,
        message: String,
    },
    CorruptMessage,
    GlobalNotFound,
}

impl core::fmt::Display for WlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::CorruptMessage => write!(f, "Message is corrupted"),
            Self::GlobalNotFound => write!(f, "requested global not found"),
            Self::Protocol { proxy, code, message } => {
                write!(f, "object: {}, code: {}, message: {}", proxy, code, message)
            },
        }
    }
}

impl From<std::io::Error> for WlError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Clone)]
pub struct Global {
    pub name: u32,
    pub interface: String,
    pub version: u32,
}

impl<S> Connection<S> {
    pub fn get_globals(&mut self) -> io::Result<()> {
        todo!()
    }

    // pub fn bind_global<I>(&mut self, version: Option<u32>) -> Result<I, BindError>
    // where
    //     I: WlInterface,
    //     S: Listener<I>,
    // {
    //     let Some(global) = self
    //         .wl_globals
    //         .iter()
    //         .find(|g| g.interface == I::interface().name)
    //     else {
    //         return Err(BindError::NotFound);
    //     };
    //
    //     let version = version.unwrap_or(global.version);
    //
    //     if version > global.version {
    //         return Err(BindError::UnsupportedVersion);
    //     }
    //
    //     // fuck the retarded borrow checker
    //     let me = self.get_mut();
    //     Ok(self
    //         .registry()
    //         .bind::<I, _>(me, global.name, &global.interface, version))
    // }
}
