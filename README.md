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
- Clap + Indicatif (CLI)

## Development
 
There are features that work only with appropriate `env secrets`. They should be set, if you want to have these features enabled.
### Features: 
- OAuth2 (`Github`, `Google`)
- Email confirmation via `Gmail`
- Telemetry (`Sentry`)

### CLI

```sh
    # Init project: generate necessities
    pnpm initp
    
    # Run dev mode
    pnpm cli dev
    
    # Generate grpc files
    pnpm cli grpc
    # or more specific prompt
    pnpm grpc:rs # for Rust
    pnpm grpc:go # for Go
    pnpm grpc:ts # for Typescript

    # Migrate db
    task migrate:db
```


### Requirements:

- [Node.js (20+)](https://nodejs.org/en)
- [Rustc (1.76+)](https://www.rust-lang.org)
- [Pnpm](https://pnpm.io/)
- [Go (1.21.2+)](https://go.dev/)
- [Docker](https://docs.docker.com/engine/)
- [Taskfile (3+)](https://taskfile.dev) (***for `db` migrations***)
- **Protobuf plugins for [Go](https://go.dev/) & [Typescript](https://www.typescriptlang.org/)** (maybe)
```sh
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
    pnpm i -g @protobuf-ts/plugin
```