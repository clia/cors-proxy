# clia-cors-proxy

A http service proxy to add CORS headers.

## Usage

clia-cors-proxy `<LISTEN ADDR>` `<LISTEN PORT>` `<FWD ADDR>` `<FWD PORT>`

```bash
clia-cors-proxy localhost 19002 localhost 19001
clia-cors-proxy localhost 19003 example.com 80
```
