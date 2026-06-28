# Matlu Website

## Development

We have developed the project on nixos so the integration with it is pretty extensive. If you have nix installed you can use the following commands:
```bash
nix develop
wrangler d1 execute matlu-dev-db --local --file=./schema.sql # first time only
start-dev-cdn
start-dev-server
```

If you do not have nix you can install everything manually:

To setup the local development environment you need to install the following things:

- [the rust toolchain cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Cloudflare Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [simple-http-server](https://crates.io/crates/simple-http-server)
- [cargo-watch](https://crates.io/crates/cargo-watch) _(optional but recommended)_

Once you have rust, cargo and wrangler installed you can install simple-http-server and cargo-watch by running:

```bash
cargo install simple-http-server cargo-watch
```

Initialize the database
```bash
wrangler d1 execute matlu-dev-db --local --file=./schema.sql # first time only
```

Then you can start the development CDN on the port `8000` with:

```bash
simple-http-server --cors --coop --coep -i ./assets
```

Then you can start the development server. The initial compilation may take up to 10 minutes, but the subsequent runs will be faster. 

```bash
cargo watch -w src -w templates -s "wrangler dev"
```


Note that if you used a node.js package manager to install wrangler you might have to run:

```bash
cargo watch -w src -w templates -s "npx wrangler dev"
```



