# <p style="text-align: center;">MaFuYu ❄️</p>

## Technologies

- Rust
- Typescript
- Go
- SvelteKit
- gRPC (+web)
- NATS
- Docker
- Caddy
- Postgres
- [Redb](https://github.com/cberner/redb)
- Redis
- GORM
- SeaORM
- Sentry
- Tracing(Rust) + Zerolog(Go)

## Development
 
There are features that work only with appropriate `env secrets`. They should be set, if you want to have these features enabled.
### Features: 
- OAuth2 (`Github`, `Google`)
- Email confirmation via `Gmail`
- Telemetry (`Sentry`)

### CLI Usage

```sh
    # init necessary files + folders 
    task cli -- init
    
    # generate grpc
    task cli -- grpc
    
    # run dev mode
    task cli -- dev
```


### Requirements:

- [Node.js (20+)](https://nodejs.org/en)
- [Taskfile (3+)](https://taskfile.dev)
- [Rustc (1.76+)](https://www.rust-lang.org)
- [Pnpm](https://pnpm.io/)
- [Go (1.21.2+)](https://go.dev/)
- [Docker](https://docs.docker.com/engine/)
