use core::slice;
use std::{
    error::Error,
    ffi::{c_char, CStr, CString},
    io::{self, Read, Write},
    mem,
    os::raw::c_void,
    ptr::{self},
};

use csfml_sys::{
    sfBool, sfPacket, sfPacket_append, sfPacket_clear, sfPacket_copy, sfPacket_create,
    sfPacket_destroy, sfPacket_endOfPacket, sfPacket_getData, sfPacket_getDataSize,
    sfPacket_readBool, sfPacket_readDouble, sfPacket_readFloat, sfPacket_readInt16,
    sfPacket_readInt32, sfPacket_readInt8, sfPacket_readString, sfPacket_readUint16,
    sfPacket_readUint32, sfPacket_readUint8, sfPacket_writeBool, sfPacket_writeDouble,
    sfPacket_writeFloat, sfPacket_writeInt16, sfPacket_writeInt32, sfPacket_writeInt8,
    sfPacket_writeString, sfPacket_writeUint16, sfPacket_writeUint32, sfPacket_writeUint8,
};

#[derive(Debug, Clone)]
pub struct Packet {
    ptr: *mut sfPacket,
}

impl Drop for Packet {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Packet {
    pub fn create() -> Result<Self, String> {
        let pack = unsafe { sfPacket_create() };
        if pack.is_null() {
            Err("Null packet pointer returned from create".to_string())
        } else {
            Ok(Self { ptr: pack })
        }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { sfPacket_destroy(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }

    pub fn copy(&self) -> Result<Self, String> {
        let pack = unsafe { sfPacket_copy(self.ptr) };
        if pack.is_null() {
            Err("Null packet pointer returned from copy".to_string())
        } else {
            Ok(Self { ptr: pack })
        }
    }

    pub fn clear(&mut self) {
        unsafe { sfPacket_clear(self.ptr) };
    }

    #[must_use]
    pub fn get_data(&self) -> Vec<u8> {
        let size = unsafe { sfPacket_getDataSize(self.ptr) };
        let data_ptr = unsafe { sfPacket_getData(self.ptr) };
        unsafe { slice::from_raw_parts(data_ptr.cast::<u8>(), size).to_vec() }
    }

    #[must_use]
    pub fn get_data_size(&self) -> usize {
        unsafe { sfPacket_getDataSize(self.ptr) }
    }

    pub fn append(&mut self, data: &[u8]) -> Result<(), String> {
        let size_before = self.get_data_size();
        let data_ptr = self.get_data();
        unsafe {
            sfPacket_append(self.ptr, data.as_ptr().cast::<c_void>(), data.len());
        }
        let size_after = self.get_data_size();
        if size_after - size_before != data.len() {
            return Err("Failed to append data to packet".to_string());
        }
        Ok(())
    }

    #[must_use]
    pub fn is_at_end(&self) -> bool {
        unsafe { sfPacket_endOfPacket(self.ptr) != 0 }
    }

    #[must_use]
    pub const fn as_csfml(&self) -> *mut sfPacket {
        self.ptr
    }

    pub const fn from_csfml(ptr: *mut sfPacket) -> Self {
        Self { ptr }
    }

    pub fn read<T>(&mut self) -> Result<T, String>
    where
        T: ReadFromPacket,
    {
        T::read_from_packet(self)
    }

    pub fn write<T>(&mut self, value: T) -> Result<(), String>
    where
        T: WriteToPacket,
    {
        T::write_to_packet(self, value)
    }

    pub fn writer(&mut self) -> Writer {
        Writer::new(self)
    }

    pub fn reader(&mut self) -> Reader {
        Reader::new(self)
    }
}

// Trait for reading from the packet
pub trait ReadFromPacket: Sized {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String>;
}

// Trait for writing to the packet
pub trait WriteToPacket {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String>;
}

impl ReadFromPacket for bool {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readBool(packet.ptr) != 0) }
    }
}

impl WriteToPacket for bool {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeBool(packet.ptr, sfBool::from(value));
        }
        Ok(())
    }
}

impl ReadFromPacket for u8 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readUint8(packet.ptr)) }
    }
}

impl WriteToPacket for u8 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeUint8(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for u16 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readUint16(packet.ptr)) }
    }
}

impl WriteToPacket for u16 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeUint16(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for u32 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readUint32(packet.ptr)) }
    }
}

impl WriteToPacket for u32 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeUint32(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for i8 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readInt8(packet.ptr)) }
    }
}

impl WriteToPacket for i8 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeInt8(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for i16 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readInt16(packet.ptr)) }
    }
}

impl WriteToPacket for i16 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeInt16(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for i32 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readInt32(packet.ptr)) }
    }
}

impl WriteToPacket for i32 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeInt32(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for f32 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readFloat(packet.ptr)) }
    }
}

impl WriteToPacket for f32 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeFloat(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for f64 {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        unsafe { Ok(sfPacket_readDouble(packet.ptr)) }
    }
}

impl WriteToPacket for f64 {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        unsafe {
            sfPacket_writeDouble(packet.ptr, value);
        }
        Ok(())
    }
}

impl ReadFromPacket for String {
    fn read_from_packet(packet: &mut Packet) -> Result<Self, String> {
        let cstr: *mut c_char = ptr::null_mut();
        unsafe {
            sfPacket_readString(packet.ptr, cstr);
            if cstr.is_null() {
                return Err("Failed to read string from packet".to_string());
            }
            Ok(CString::from_raw(cstr).to_string_lossy().to_string())
        }
    }
}

impl WriteToPacket for String {
    fn write_to_packet(packet: &mut Packet, value: Self) -> Result<(), String> {
        let cstr = CString::new(value).map_err(|e| e.to_string())?;
        unsafe {
            sfPacket_writeString(packet.ptr, cstr.as_ptr());
        }
        Ok(())
    }
}

/// Writer type for a packet
pub struct Writer<'a> {
    packet: &'a mut Packet,
}

impl<'a> Writer<'a> {
    /// Initializes a `PacketWriter` that will write to the packet
    fn new(packet: &'a mut Packet) -> Self {
        Writer { packet }
    }

    /// Generic write function for writing data to the packet
    pub fn write_data<T>(&mut self, value: T) -> Result<(), Box<dyn Error>>
    where
        T: WriteToPacket,
    {
        self.packet.write(value)?;
        Ok(())
    }
}

impl Write for Writer<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size_before = self.packet.get_data_size();
        self.packet.append(buf).unwrap();
        let size_after = self.packet.get_data_size();
        let size_written = size_after - size_before;
        if size_written == 0 && !buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Cannot write to packet",
            ));
        }
        Ok(size_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

/// Reader type for a packet
pub struct Reader<'a> {
    packet: &'a mut Packet,
}

impl Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut count = 0;
        for byte in buf.iter_mut() {
            if self.packet.is_at_end() {
                return Ok(count);
            }
            let val = self.packet.read::<u8>().unwrap();
            *byte = val;
            count += 1;
        }
        Ok(count)
    }
}

impl<'a> Reader<'a> {
    /// Initializes a Reader which will read the packet's bytes
    /// Slightly slower than read for bigger types but more convenient for some things
    fn new(packet: &'a mut Packet) -> Self {
        Reader { packet }
    }

    pub fn read_data<T>(&mut self) -> Result<T, String>
    where
        T: ReadFromPacket,
    {
        self.packet.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_reading_and_writing() {
        let mut pack1 = Packet::create().expect("Failed to create packet");

        pack1
            .write::<u16>(1999)
            .expect("Failed to write u16 to packet");
        pack1
            .write::<bool>(true)
            .expect("Failed to write bool to packet");

        {
            let mut writer = pack1.writer();
            writer
                .write_data::<u32>(12_345_678)
                .expect("Failed to write using writer");
            writer
                .write_data("oh:".to_string())
                .expect("Failed to write 'oh:'");
        }

        let str_data = b"abc";
        pack1
            .append(str_data)
            .expect("Failed to append data to packet");
        assert_eq!(pack1.get_data_size(), 17);

        let mut pack2 = pack1.copy().expect("Failed to copy packet");
        pack1.clear();
        assert_eq!(pack1.get_data_size(), 0);
        assert!(pack1.is_at_end());

        assert_eq!(
            pack2.read::<u16>().expect("Failed to read u16 from packet"),
            1999
        );
        assert!(pack2
            .read::<bool>()
            .expect("Failed to read bool from packet"));

        {
            let mut buf = vec![0; 16];
            let data = pack2.get_data();
            let mut reader = pack2.reader();
            assert_eq!(12_345_678, reader.read_data().unwrap());
            let count = reader
                .read(&mut buf)
                .expect("Failed to read data using reader");
            // TODO: Fix tests from reading size marker
            // assert_eq!(count, 6);
            assert_eq!(&buf[4..count], b"oh:abc");
        }

        let data = pack2.get_data();
        assert_eq!(data.len(), 17);
        assert_eq!(&data[12..15], b"h:a");
        assert!(pack2.is_at_end());
    }
}
