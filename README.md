# clia-cors-proxy

A http service proxy to add CORS headers.

## Usage

clia-cors-proxy `<LISTEN ADDR>` `<LISTEN PORT>` `<FWD ADDR>` `<FWD PORT>`

Examples:

```bash
clia-cors-proxy localhost 19002 localhost 19001
clia-cors-proxy localhost 19003 example.com 80
```

## Changelog

- Version 0.2.0: Direct respond for OPTIONS requests.
- Version 0.1.1: Improve log output.
- Version 0.1.0: Initial version.
