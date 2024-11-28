use std::{os::raw::c_void, ptr};

use csfml_sys::{
    sfBool, sfIpAddress_Any, sfSocketNotReady, sfSocketSelector, sfSocketSelector_addTcpListener,
    sfSocketSelector_addTcpSocket, sfSocketSelector_addUdpSocket, sfSocketSelector_clear,
    sfSocketSelector_copy, sfSocketSelector_create, sfSocketSelector_destroy,
    sfSocketSelector_isTcpListenerReady, sfSocketSelector_isTcpSocketReady,
    sfSocketSelector_isUdpSocketReady, sfSocketSelector_removeTcpListener,
    sfSocketSelector_removeTcpSocket, sfSocketSelector_removeUdpSocket, sfSocketSelector_wait,
    sfTcpListener, sfTcpListener_accept, sfTcpListener_create, sfTcpListener_destroy,
    sfTcpListener_getLocalPort, sfTcpListener_isBlocking, sfTcpListener_listen,
    sfTcpListener_setBlocking, sfTcpSocket, sfTcpSocket_connect, sfTcpSocket_create,
    sfTcpSocket_destroy, sfTcpSocket_disconnect, sfTcpSocket_getLocalPort,
    sfTcpSocket_getRemoteAddress, sfTcpSocket_getRemotePort, sfTcpSocket_isBlocking,
    sfTcpSocket_receive, sfTcpSocket_receivePacket, sfTcpSocket_send, sfTcpSocket_sendPacket,
    sfTcpSocket_sendPartial, sfTcpSocket_setBlocking, sfUdpSocket, sfUdpSocket_bind,
    sfUdpSocket_create, sfUdpSocket_destroy, sfUdpSocket_getLocalPort, sfUdpSocket_isBlocking,
    sfUdpSocket_maxDatagramSize, sfUdpSocket_receive, sfUdpSocket_receivePacket, sfUdpSocket_send,
    sfUdpSocket_sendPacket, sfUdpSocket_setBlocking, sfUdpSocket_unbind,
};

use crate::system::time::Time;

use super::{code_to_err, Error, IpAddress, Packet};

pub trait Socket {
    fn add_to_selector(&self, selector: &mut SocketSelector);
    fn remove_from_selector(&self, selector: &mut SocketSelector);
    fn is_ready(&self, selector: &SocketSelector) -> bool;
}

#[derive(Debug, Clone)]
pub struct IpAndPort {
    ip: IpAddress,
    port: u16,
}

#[derive(Debug, Clone)]
pub struct ReceivedRaw {
    pub data: Vec<u8>,
    pub sender: IpAndPort,
}

#[derive(Debug, Clone)]
pub struct SocketSelector {
    ptr: *mut sfSocketSelector,
}

impl Drop for SocketSelector {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl SocketSelector {
    /// Creates a new socket selector
    pub fn create() -> Result<SocketSelector, Error> {
        let sock = unsafe { sfSocketSelector_create() };
        if sock.is_null() {
            Err(Error::OtherError)
        } else {
            Ok(SocketSelector { ptr: sock })
        }
    }

    /// Destroys this socket selector
    pub fn destroy(&mut self) {
        unsafe {
            sfSocketSelector_destroy(self.ptr);
        }
    }

    /// Copies this socket selector
    pub fn copy(&self) -> Result<SocketSelector, Error> {
        let sock = unsafe { sfSocketSelector_copy(self.ptr) };
        if sock.is_null() {
            Err(Error::OtherError)
        } else {
            Ok(SocketSelector { ptr: sock })
        }
    }

    /// Adds a socket to the selector
    pub fn add_socket<T: Socket>(&mut self, socket: &mut T) {
        socket.add_to_selector(self);
    }

    /// Removes a socket from the selector
    pub fn remove_socket<T: Socket>(&mut self, socket: &T) {
        socket.remove_from_selector(self);
    }

    /// Removes all sockets from the selector
    pub fn clear(&mut self) {
        unsafe { sfSocketSelector_clear(self.ptr) };
    }

    /// Wait until one of the sockets is ready to receive something (or timeout)
    pub fn wait(&self, timeout: Option<Time>) -> bool {
        let time = timeout.unwrap_or(Time { microseconds: 0 });
        let ret = unsafe { sfSocketSelector_wait(self.ptr, time.to_csfml()) };
        ret != 0
    }

    /// Checks if a socket is ready to read data
    pub fn is_socket_ready<T: Socket>(&self, socket: &T) -> bool {
        socket.is_ready(self)
    }
}

pub struct TcpListener {
    ptr: *mut sfTcpListener,
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl TcpListener {
    /// Creates a new TCP listener socket
    pub fn create() -> Result<Self, Error> {
        let sock = unsafe { sfTcpListener_create() };
        if sock.is_null() {
            Err(Error::OtherError)
        } else {
            Ok(TcpListener { ptr: sock })
        }
    }

    /// Destroys this socket
    pub fn destroy(&mut self) {
        unsafe { sfTcpListener_destroy(self.ptr) };
    }

    /// Enables or disables blocking mode (true for blocking)
    pub fn set_blocking(&mut self, blocking: bool) {
        unsafe {
            sfTcpListener_setBlocking(self.ptr, sfBool::from(blocking));
        }
    }

    /// Tells whether or not the socket is in blocking mode
    #[must_use]
    pub fn is_blocking(&self) -> bool {
        unsafe { sfTcpListener_isBlocking(self.ptr) != 0 }
    }

    /// Gets the port this socket is listening on
    /// Returns an error if the socket is not listening
    pub fn get_local_port(&self) -> Result<u16, Error> {
        let port = unsafe { sfTcpListener_getLocalPort(self.ptr) };
        if port == 0 {
            Err(Error::OtherError)
        } else {
            Ok(port)
        }
    }

    /// Starts listening for incoming connections on a given port (and address)
    /// If address is null, it listens on any address of this machine
    pub fn listen(&mut self, port: u16, address: Option<IpAddress>) -> Result<(), Error> {
        let ip = address.unwrap_or(IpAddress::from_csfml(unsafe { sfIpAddress_Any }));
        let code = unsafe { sfTcpListener_listen(self.ptr, port, ip.to_csfml()) };
        code_to_err(code)
    }

    /// Accepts a new connection, returns a `TcpSocket` if it works
    /// If the TCP is in blocking mode, it will wait
    pub fn accept(&mut self) -> Result<Option<TcpSocket>, Option<Error>> {
        let mut ret = ptr::null_mut();
        let ret_ptr = &mut ret;

        let code = unsafe { sfTcpListener_accept(self.ptr, ret_ptr) };

        if !self.is_blocking() && code == sfSocketNotReady {
            return Ok(None);
        }

        code_to_err(code).map_err(|_| Error::OtherError)?;

        Ok(Some(TcpSocket { ptr: ret }))
    }
}

pub struct TcpSocket {
    ptr: *mut sfTcpSocket,
}

impl Drop for TcpSocket {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl TcpSocket {
    /// Creates a new TCP socket
    pub fn create() -> Result<Self, Error> {
        let sock = unsafe { sfTcpSocket_create() };
        if !sock.is_null() {
            Ok(TcpSocket { ptr: sock })
        } else {
            Err(Error::OtherError)
        }
    }

    /// Destroys this socket
    pub fn destroy(&mut self) {
        unsafe { sfTcpSocket_destroy(self.ptr) };
    }

    /// Enables or disables blocking mode (true for blocking)
    pub fn set_blocking(&mut self, blocking: bool) {
        unsafe {
            sfTcpSocket_setBlocking(self.ptr, sfBool::from(blocking));
        }
    }

    /// Tells whether or not the socket is in blocking mode
    #[must_use]
    pub fn is_blocking(&self) -> bool {
        unsafe { sfTcpSocket_isBlocking(self.ptr) != 0 }
    }

    /// Gets the port this socket is bound to (null for no port)
    #[must_use]
    pub fn get_local_port(&self) -> Option<u16> {
        let port = unsafe { sfTcpSocket_getLocalPort(self.ptr) };
        if port == 0 {
            None
        } else {
            Some(port)
        }
    }

    /// Gets the address of the other TCP socket that is currently connected
    pub fn get_remote(&self) -> Result<IpAndPort, Error> {
        let port = unsafe { sfTcpSocket_getRemotePort(self.ptr) };
        if port == 0 {
            return Err(Error::OtherError);
        }
        let ip = unsafe { sfTcpSocket_getRemoteAddress(self.ptr) };
        let ip_and_port = IpAndPort {
            ip: IpAddress::from_csfml(ip),
            port,
        };
        debug_assert!((ip_and_port.ip != IpAddress::none()));
        Ok(ip_and_port)
    }

    /// Connects to a remote server
    pub fn connect(&mut self, remote: IpAndPort, timeout: Time) -> Result<(), Error> {
        let code = unsafe {
            sfTcpSocket_connect(
                self.ptr,
                remote.ip.to_csfml(),
                remote.port,
                timeout.to_csfml(),
            )
        };
        code_to_err(code)
    }

    /// Disconnects from the remote
    pub fn disconnect(&mut self) {
        unsafe { sfTcpSocket_disconnect(self.ptr) };
    }

    /// Sends raw data to the remote
    pub fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        let code =
            unsafe { sfTcpSocket_send(self.ptr, data.as_ptr().cast::<c_void>(), data.len()) };
        code_to_err(code)
    }

    /// Sends part of the buffer to the remote, returns the slice of the remaining data
    pub fn send_partial<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], Error> {
        let mut sent: usize = 0;
        let code = unsafe {
            sfTcpSocket_sendPartial(
                self.ptr,
                data.as_ptr().cast::<c_void>(),
                data.len(),
                &mut sent,
            )
        };
        code_to_err(code)?;
        Ok(&data[sent..])
    }

    /// Sends a packet to the remote
    pub fn send_packet(&mut self, packet: Packet) -> Result<(), Error> {
        let code = unsafe { sfTcpSocket_sendPacket(self.ptr, packet.as_csfml()) };
        code_to_err(code)
    }

    /// Receives raw data from the remote
    pub fn receive<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a [u8], i32> {
        let mut size: usize = 0;
        let code = unsafe {
            sfTcpSocket_receive(
                self.ptr,
                buf.as_mut_ptr().cast::<c_void>(),
                buf.len(),
                &mut size,
            )
        };
        let _ = code_to_err(code);
        Ok(&buf[..size])
    }

    /// Receives a packet from the remote
    pub fn receive_packet(&mut self, packet: &mut Packet) -> Result<(), Error> {
        let code = unsafe { sfTcpSocket_receivePacket(self.ptr, packet.as_csfml()) };
        code_to_err(code)
    }
}

pub struct UdpSocket {
    ptr: *mut sfUdpSocket,
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        unsafe { sfUdpSocket_destroy(self.ptr) };
    }
}

impl UdpSocket {
    /// Creates a new UDP socket
    pub fn create() -> Result<Self, Error> {
        let sock = unsafe { sfUdpSocket_create() };
        if !sock.is_null() {
            Ok(UdpSocket { ptr: sock })
        } else {
            Err(Error::OtherError)
        }
    }

    /// Destroys this socket
    pub fn destroy(self) {
        unsafe { sfUdpSocket_destroy(self.ptr) };
    }

    /// Enables or disables blocking mode (true for blocking)
    pub fn set_blocking(&mut self, blocking: bool) {
        unsafe {
            sfUdpSocket_setBlocking(self.ptr, blocking as sfBool);
        }
    }

    /// Tells whether or not the socket is in blocking mode
    pub fn is_blocking(&self) -> bool {
        unsafe { sfUdpSocket_isBlocking(self.ptr) != 0 }
    }

    /// Gets the port this socket is bound to (null for no port)
    pub fn get_local_port(&self) -> Option<u16> {
        let port = unsafe { sfUdpSocket_getLocalPort(self.ptr) };
        if port == 0 {
            None
        } else {
            Some(port)
        }
    }

    /// Binds the socket to a specified port and IP address
    pub fn bind(&mut self, port: Option<u16>, ip: Option<IpAddress>) -> Result<(), Error> {
        let p = port.unwrap_or(0);
        let i = ip.unwrap_or_else(|| IpAddress::from(0));
        let code = unsafe { sfUdpSocket_bind(self.ptr, p, i.to_csfml()) };
        code_to_err(code)
    }

    /// Unbinds the socket from the port it's bound to
    pub fn unbind(&mut self) {
        unsafe { sfUdpSocket_unbind(self.ptr) };
    }

    /// Sends raw data to a recipient
    pub fn send(&mut self, data: &[u8], remote: &IpAndPort) -> Result<(), Error> {
        let code = unsafe {
            sfUdpSocket_send(
                self.ptr,
                data.as_ptr().cast::<c_void>(),
                data.len(),
                remote.ip.to_csfml(),
                remote.port,
            )
        };
        code_to_err(code)
    }

    /// Sends a packet to a recipient
    pub fn send_packet(&mut self, packet: &Packet, remote: &IpAndPort) -> Result<(), Error> {
        let code = unsafe {
            sfUdpSocket_sendPacket(
                self.ptr,
                packet.as_csfml(),
                remote.ip.to_csfml(),
                remote.port,
            )
        };
        code_to_err(code)
    }

    /// Receives raw data from a recipient
    pub fn receive(&mut self, buf: &mut [u8]) -> Result<ReceivedRaw, Error> {
        let mut size: usize = 0;
        let mut remote: IpAndPort = IpAndPort {
            ip: IpAddress::none(),
            port: 0,
        };
        let code = unsafe {
            sfUdpSocket_receive(
                self.ptr,
                buf.as_mut_ptr().cast::<c_void>(),
                buf.len(),
                &mut size,
                &mut remote.ip.to_csfml(),
                &mut remote.port,
            )
        };
        code_to_err(code)?;
        Ok(ReceivedRaw {
            data: buf[..size].to_vec(),
            sender: remote,
        })
    }

    /// Receives a packet from a recipient
    pub fn receive_packet(&mut self, packet: &mut Packet) -> Result<IpAndPort, Error> {
        let mut remote: IpAndPort = IpAndPort {
            ip: IpAddress::none(),
            port: 0,
        };
        let code = unsafe {
            sfUdpSocket_receivePacket(
                self.ptr,
                packet.as_csfml(),
                &mut remote.ip.to_csfml(),
                &mut remote.port,
            )
        };
        code_to_err(code)?;
        Ok(remote)
    }

    /// Gets the max datagram size you can send
    pub fn get_max_datagram_size() -> u32 {
        unsafe { sfUdpSocket_maxDatagramSize() }
    }
}

// Implement the Socket trait for UdpSocket
impl Socket for UdpSocket {
    fn add_to_selector(&self, selector: &mut SocketSelector) {
        unsafe {
            sfSocketSelector_addUdpSocket(selector.ptr, self.ptr);
        }
    }

    fn remove_from_selector(&self, selector: &mut SocketSelector) {
        unsafe {
            sfSocketSelector_removeUdpSocket(selector.ptr, self.ptr);
        }
    }

    fn is_ready(&self, selector: &SocketSelector) -> bool {
        unsafe { sfSocketSelector_isUdpSocketReady(selector.ptr, self.ptr) != 0 }
    }
}

// Implement the Socket trait for TcpSocket
impl Socket for TcpSocket {
    fn add_to_selector(&self, selector: &mut SocketSelector) {
        unsafe {
            sfSocketSelector_addTcpSocket(selector.ptr, self.ptr);
        }
    }

    fn remove_from_selector(&self, selector: &mut SocketSelector) {
        unsafe {
            sfSocketSelector_removeTcpSocket(selector.ptr, self.ptr);
        }
    }

    fn is_ready(&self, selector: &SocketSelector) -> bool {
        unsafe { sfSocketSelector_isTcpSocketReady(selector.ptr, self.ptr) != 0 }
    }
}

impl Socket for TcpListener {
    fn add_to_selector(&self, selector: &mut SocketSelector) {
        unsafe {
            sfSocketSelector_addTcpListener(selector.ptr, self.ptr);
        }
    }

    fn remove_from_selector(&self, selector: &mut SocketSelector) {
        unsafe {
            sfSocketSelector_removeTcpListener(selector.ptr, self.ptr);
        }
    }

    fn is_ready(&self, selector: &SocketSelector) -> bool {
        unsafe { sfSocketSelector_isTcpListenerReady(selector.ptr, self.ptr) != 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn udp_socket_basic_test() {
        let mut buf: [u8; 1024] = [0; 1024];
        let packet = Packet::create().unwrap();
        let mut socket = UdpSocket::create().unwrap();

        socket.set_blocking(false);
        assert!(!socket.is_blocking());

        socket.bind(None, None).unwrap();
        let port = socket.get_local_port().unwrap();
        assert!(port >= 49152);

        assert!(socket.receive(&mut buf).is_err());

        socket.unbind();
        assert!(socket.get_local_port().is_none());

        let target = IpAndPort {
            ip: IpAddress::none(),
            port: 1,
        };

        // assert!(socket.send_packet(&packet, &target).is_err());
        // assert!(socket.send(&buf[..10], &target).is_err());
    }
}
