# snippits-rs
A web app written in rust where users can paste snippits and vote on them.


# Setup 
### Install rust
* Follow rustup install instructions: https://www.rust-lang.org/en-US/install.html
* The dependency on rocket requires rust nightly to be installed. Follow their instructions: https://rocket.rs/guide/getting-started/ 

### Install Redis
* Follow redis install instructions: https://redis.io/topics/quickstart

### How to run
* Start redis e.g. `redis-server /usr/local/etc/redis.conf`
* `cargo run`
