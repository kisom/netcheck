//! netcheck is a tool to determine if we have a functioning network
//! connection.
#![feature(lookup_host)]

/// A Server pairs a host and port.
#[derive(Clone, Copy, Debug)]
struct Server {
    host: &'static str,
    port: u16,
}

impl Server {
    fn address(&self) -> String {
        return format!("{}:{}", self.host, self.port);
    }
}

/// The SERVERS were chosen as two that I own that are likely to
/// always be working, and one public address in the event that I've
/// managed to botch my infrastructure.
const SERVERS: [Server; 3] = [
    Server {
        host: "kyleisom.net",
        port: 443,
    },
    Server {
        host: "svc.dropsonde.net",
        port: 22,
    },
    Server {
        host: "google.com",
        port: 443,
    },
];

/// can_resolve_host attempts to resolve the hosts in SERVERS. The
/// function is short-circuiting; intent is to determine if the
/// network is down, thus being able to resolve any of the servers is
/// indicative of a functioning network connection.
fn can_resolve_host() -> bool {
    for srv in SERVERS.iter() {
        match std::net::lookup_host(srv.host) {
            Ok(_) => return true,
            Err(_) => continue,
        }
    }

    return false;
}

/// can_connect_to_host checks to see if any of the SERVERS are
/// reachable. This is a short-circuiting function; the intent is to
/// figure out if the network is down, so reaching any of the servers
/// is indicative of a functioning network connection.
fn can_connect_to_host() -> bool {
    for srv in SERVERS.iter() {
        match std::net::TcpStream::connect(srv.address()) {
            Ok(_) => return true,
            Err(_) => continue,
        }
    }

    return false;
}

/// is_network_down attempts to answer that very same question. It
/// first attempts to resolve host names, then attempts to connect
/// to the servers.
fn is_network_down() -> bool {
    print!("checking DNS... ");
    if !can_resolve_host() {
        println!("failed!");
        return true;
    }
    println!("OK");

    print!("checking TCP connections... ");
    if !can_connect_to_host() {
        println!("failed!");
        return true;
    }
    println!("OK");
    return false;
}

fn main() {
    if is_network_down() {
        std::process::exit(1);
    }
}
