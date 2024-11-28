use std::{error::Error, ffi::CString, fmt::Display, mem, str::FromStr};

use csfml_sys::{
    sfIpAddress, sfIpAddress_fromBytes, sfIpAddress_fromInteger, sfIpAddress_fromString,
    sfIpAddress_getLocalAddress, sfIpAddress_getPublicAddress, sfIpAddress_toInteger,
};

use crate::system::time::Time;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpAddress {
    pub address: [i8; 16],
}

impl From<u32> for IpAddress {
    fn from(value: u32) -> Self {
        unsafe { Self::from_csfml(sfIpAddress_fromInteger(value)) }
    }
}

impl TryFrom<&str> for IpAddress {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let string = CString::from_str(value)?;
        Ok(unsafe { Self::from_csfml(sfIpAddress_fromString(string.as_ptr())) })
    }
}

impl IpAddress {
    #[must_use]
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        unsafe { Self::from_csfml(sfIpAddress_fromBytes(a, b, c, d)) }
    }

    #[must_use]
    pub fn none() -> Self {
        Self::from(0)
    }

    #[must_use]
    pub fn to_csfml(self) -> sfIpAddress {
        unsafe { mem::transmute(self) }
    }

    #[must_use]
    pub fn from_csfml(value: sfIpAddress) -> Self {
        unsafe { mem::transmute(value) }
    }

    #[must_use]
    pub fn local_address() -> Self {
        unsafe { Self::from_csfml(sfIpAddress_getLocalAddress()) }
    }

    pub fn public_address(timeout: Option<Time>) -> Result<Self, Box<dyn Error>> {
        let time = timeout.unwrap_or(Time { microseconds: 0 });
        let ip = unsafe { Self::from_csfml(sfIpAddress_getPublicAddress(time.to_csfml())) };

        if ip == Self::none() {
            return Err("Timeout".into());
        }

        Ok(ip)
    }

    /// Converts the `IpAddress` to a slice of its raw bytes
    #[must_use]
    pub const fn bytes(&self) -> &[i8] {
        &self.address
    }

    /// Converts the `IpAddress` to an integer
    #[must_use]
    pub fn to_int(self) -> u32 {
        unsafe { sfIpAddress_toInteger(self.to_csfml()) }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let slice = self.bytes();
            let mut parts = vec![];

            for chunk in slice.iter().rev().take(4).rev() {
                parts.push(format!("{}", *chunk as u8 as char));
            }

            parts.join(".")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipaddress_test() {
        let mut ip = IpAddress::new(0x01, 0x23, 0x45, 0x67);
        assert_eq!(ip.to_int(), 0x0123_4567);
        ip = IpAddress::from(0xabab_abab);
        assert_eq!(ip, IpAddress::new(0xab, 0xab, 0xab, 0xab));
        ip = IpAddress::try_from("localhost").unwrap();
        assert_eq!(ip.to_string(), "\0.\0.\0.\0");
    }
}
