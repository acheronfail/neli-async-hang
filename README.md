# `neli-nl80211-hang`

A small repository to reproduce a bug in `neli`.

## Usage

```bash
# first, get the interface of your wireless card with:
ip addr

# then, run this with the interface index:
cargo run -- <interface_index>
```
