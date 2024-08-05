use std::net::TcpStream;
use ssh2::Session;
use query_wmi::WMIConnection;
use query_wmi::COMLibrary;
use reqwest::Client;

pub enum Connection {
    Tcp(TcpStream),
    Ssh(Session),
    Wmi(WMIConnection),
    Http(reqwest::Client),
    Https(reqwest::Client),
    // Add more as needed
}

pub fn establish_connection(host: &str, protocol: &str) -> Result<Connection, Box<dyn std::error::Error>> {
    match protocol {
        "tcp" => {
            let stream = TcpStream::connect(host)?;
            Ok(Connection::Tcp(stream))
        },
        "ssh" => {
            let tcp = TcpStream::connect(host)?;
            let mut sess = Session::new()?;
            sess.set_tcp_stream(tcp);
            sess.handshake()?;
            Ok(Connection::Ssh(sess))
        },
        "wmi" => {
            let com_con = COMLibrary::new()?;
            let wmi_con = WMIConnection::new(com_con.into())?;
            Ok(Connection::Wmi(wmi_con))
        },
        "http" => {
            let client = Client::builder().build()?;
            Ok(Connection::Http(client))
        },
        "https" => {
            let client = Client::builder()
                .use_rustls_tls()
                .build()?;
            Ok(Connection::Https(client))
        },
        // Add more protocols as needed
        _ => Err("Unsupported protocol".into()),
    }
}