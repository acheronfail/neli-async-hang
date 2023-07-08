# `neli-nl80211-hang`

A small repository to reproduce a bug in `neli`.

## Usage

```bash
# HAPPY PATH:
# first, get the interface of your wireless card with:
ip addr
# then, run this with the interface index:
cargo run -- <interface_index>
# it works!

# BUG PATH:
# run this with something that IS NOT WIRELESS
# using your loopback interface will do just fine
cargo run -- 1
```
