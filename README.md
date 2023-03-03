# clia-cors-proxy

A http service proxy to add CORS headers.

## Install

1. Install Rust:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install this:

```shell
cargo install clia-cors-proxy
```

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

## 原理说明

这个就是开一个代理服务，这个服务就是专门给代理的请求加一个CORS跨域的头。你要访问所需的接口，先访问这个服务，这个服务代理去访问那个接口，然后在返回的响应消息上加一个CORS跨域的头，这样浏览器就可以读取到返回的数据内容了。
