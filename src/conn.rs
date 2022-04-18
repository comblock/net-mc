use std::io::{self, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;
use super::packet::*;


pub struct Conn {
    pub host: SocketAddr,
    pub stream: TcpStream,
    pub writer : io::BufWriter<TcpStream>,
    pub reader : io::BufReader<TcpStream>,
    pub threshhold: i32,
}

impl io::Write for Conn {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl io::Read for Conn {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl Conn {
    pub fn connect(addr: SocketAddr) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream.try_clone()?);
        Ok(Self {
            host: addr,
            stream,
            writer,
            reader,
            threshhold: -1,
        })
    }

    pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> anyhow::Result<Self> {
        let stream = TcpStream::connect_timeout(addr, timeout)?;
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream.try_clone()?);
        Ok(Self {
            host: *addr,
            stream,
            writer,
            reader,
            threshhold: -1,
        })
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        self.stream.shutdown(Shutdown::Both)
    }

    pub fn send_packet(&mut self, packet: &impl Packet) -> anyhow::Result<()> {
        packet.encode()?.pack(self, self.threshhold)?;
        self.flush()?;
        Ok(())
    }
}