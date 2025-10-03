//! Low-level socket connection management

use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::process::Command;
use serde_json::Value;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::error::{GuiError, Result};

/// Generate a random address for abstract namespace sockets
pub fn generate_random_address() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect()
}

/// Bind an abstract namespace Unix socket
pub fn bind_abstract_socket(name: &str) -> Result<UnixListener> {
    unsafe {
        use std::os::unix::io::FromRawFd;
        
        let fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
        if fd < 0 {
            return Err(GuiError::Io(std::io::Error::last_os_error()));
        }
        
        let mut addr_bytes = vec![0u8];
        addr_bytes.extend_from_slice(name.as_bytes());
        
        let mut addr: libc::sockaddr_un = std::mem::zeroed();
        addr.sun_family = libc::AF_UNIX as u16;
        for (i, &byte) in addr_bytes.iter().enumerate() {
            addr.sun_path[i] = byte as _;
        }
        
        let addr_len = (std::mem::size_of::<libc::sa_family_t>() + addr_bytes.len()) as libc::socklen_t;
        
        if libc::bind(fd, &addr as *const _ as *const libc::sockaddr, addr_len) < 0 {
            libc::close(fd);
            return Err(GuiError::SocketBind(format!("bind failed for {}", name)));
        }
        
        if libc::listen(fd, 1) < 0 {
            libc::close(fd);
            return Err(GuiError::SocketBind(format!("listen failed for {}", name)));
        }
        
        Ok(UnixListener::from_raw_fd(fd))
    }
}

/// Send a JSON message to a stream
pub fn send_message(stream: &mut UnixStream, msg: &Value) -> Result<()> {
    let json_bytes = msg.to_string().into_bytes();
    stream.write_all(&(json_bytes.len() as u32).to_be_bytes())?;
    stream.write_all(&json_bytes)?;
    stream.flush()?;
    Ok(())
}

/// Read a JSON message from a stream
pub fn read_message(stream: &mut UnixStream) -> Result<Value> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;
    
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    
    let value = serde_json::from_slice(&buf)?;
    Ok(value)
}

/// Send a message and read the response
pub fn send_and_read(stream: &mut UnixStream, msg: &Value) -> Result<Value> {
    eprintln!("[DEBUG] send_and_read: sending...");
    send_message(stream, msg)?;
    eprintln!("[DEBUG] send_and_read: reading response...");
    let response = read_message(stream)?;
    eprintln!("[DEBUG] send_and_read: got response!");
    Ok(response)
}

/// Connection to Termux GUI service
pub struct Connection {
    pub main_stream: UnixStream,
    pub event_stream: UnixStream,
}

impl Connection {
    /// Create a new connection to Termux GUI service
    pub fn new() -> Result<Self> {
        eprintln!("[DEBUG] Generating addresses...");
        let addr_main = generate_random_address();
        let addr_event = generate_random_address();
        
        eprintln!("[DEBUG] Binding sockets...");
        let main_listener = bind_abstract_socket(&addr_main)?;
        let event_listener = bind_abstract_socket(&addr_event)?;
        
        eprintln!("[DEBUG] Sending broadcast...");
        // Try termux-am first, fall back to am
        let output = Command::new("termux-am")
            .args(&[
                "broadcast",
                "-n",
                "com.termux.gui/.GUIReceiver",
                "--es",
                "mainSocket",
                &addr_main,
                "--es",
                "eventSocket",
                &addr_event,
            ])
            .output();
        
        match output {
            Ok(out) if !out.status.success() => {
                eprintln!("[DEBUG] termux-am failed, trying am...");
                Command::new("am")
                    .args(&[
                        "broadcast",
                        "-n",
                        "com.termux.gui/.GUIReceiver",
                        "--es",
                        "mainSocket",
                        &addr_main,
                        "--es",
                        "eventSocket",
                        &addr_event,
                    ])
                    .output()
                    .map_err(|_| GuiError::ConnectionFailed)?;
            }
            Err(_) => {
                eprintln!("[DEBUG] termux-am not found, trying am...");
                Command::new("am")
                    .args(&[
                        "broadcast",
                        "-n",
                        "com.termux.gui/.GUIReceiver",
                        "--es",
                        "mainSocket",
                        &addr_main,
                        "--es",
                        "eventSocket",
                        &addr_event,
                    ])
                    .output()
                    .map_err(|_| GuiError::ConnectionFailed)?;
            }
            _ => {}
        }
        
        eprintln!("[DEBUG] Accepting connections...");
        let (mut main_stream, _) = main_listener.accept()?;
        let (event_stream, _) = event_listener.accept()?;
        
        eprintln!("[DEBUG] Handshake...");
        // Protocol handshake
        main_stream.write_all(&[0x01])?;
        main_stream.read_exact(&mut [0u8; 1])?;
        
        eprintln!("[DEBUG] Connection established!");
        Ok(Connection {
            main_stream,
            event_stream,
        })
    }
    
    /// Send a message without waiting for response
    pub fn send(&mut self, msg: &Value) -> Result<()> {
        send_message(&mut self.main_stream, msg)
    }
    
    /// Send a message and read the response
    pub fn send_read(&mut self, msg: &Value) -> Result<Value> {
        send_and_read(&mut self.main_stream, msg)
    }
    
    /// Get a mutable reference to the event stream
    pub fn event_stream(&mut self) -> &mut UnixStream {
        &mut self.event_stream
    }
}
