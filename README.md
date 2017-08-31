# netcheck

This is a simple utility to check whether I have a network connection. I
run it on carbon, my Jetson TX-2:

![Photo of the Jetson TX-2 sitting on my desk](https://p.kyleisom.net/i/jetson.jpg)

This is an ARM64 SBC with an 256-core Nvidia GPU in it that lives
tucked away in my garage. It uses a reverse tunnel through a VPS for
remote access, but the connection is over WiFi. Occasionally, the WiFi
dies and I lose access to the machine. This runs in a cron job:

```
*/5	*	*	*	*	netcheck || reconnect
```

I could have done all this in a shell script, but it was a good
opportunity to learn some network programming in Rust.

NB: as of 2017-08-31, this requires nightly rust for the
[`std::net::lookup_host`](https://doc.rust-lang.org/std/net/fn.lookup_host.html)
function.
