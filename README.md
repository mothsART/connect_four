# Connect Four

[![Build Status](https://travis-ci.org/mothsART/connect_four.png?branch=master)](https://travis-ci.org/mothsART/connect_four)

[![LICENSE](https://img.shields.io/badge/license-BSD-blue.svg)](LICENSE)

a websocket Game Prototype

## Installation

```bash
  git clone https://github.com/mothsART/connect_four.git
```

## Launch

### Web Server (HTTP)

```bash
  cargo run --bin web
```

### WebSocket Server

```bash
  cargo run --bin ws
```

## Dev Mode

### VueJS : watch mode

```bash
  cd static
  npm run dev
```

### Unit test

```bash
  cargo test
  cargo test grid
```
