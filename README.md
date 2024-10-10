# <p style="text-align: center;">MaFuYu ❄️</p>

## Technologies:

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

## Development:

There are features that work only with appropriate `env secrets`. They should be set, if you want to have these features enabled.

### Features:

- OAuth2 (`Github`, `Google`)
- Email confirmation via `Gmail`
- Telemetry (`Sentry`)

### CLI

```sh
    # Init project: generate necessities
    bun initp

    # Run dev mode
    bun cli dev

    # Generate grpc files
    bun cli grpc
    # or more specific prompt
    bun grpc:rs # for Rust
    bun grpc:go # for Go
    bun grpc:ts # for Typescript

    # Migrate db
    task db:migrate
```

### Requirements:

- [Node.js (20+)](https://nodejs.org/en)
- [Rustc (1.76+)](https://www.rust-lang.org)
- [Bun](https://bun.sh/)
- [Go (1.21.2+)](https://go.dev/)
- [Docker](https://docs.docker.com/engine/)
- [Taskfile (3+)](https://taskfile.dev) (**_for `db` migrations_**)
- **Protobuf plugins for [Go](https://go.dev/) & [Typescript](https://www.typescriptlang.org/)** (maybe)

```sh
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
    bun i -g @protobuf-ts/plugin
```
