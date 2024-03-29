module github.com/dehwyy/mafuyu/apps/mailer

go 1.21.2

replace github.com/dehwyy/mafuyu/libs/nats => ../../libs/nats

replace github.com/dehwyy/mafuyu/libs/logger => ../../libs/logger

replace github.com/dehwyy/mafuyu/libs/config => ../../libs/config

replace github.com/dehwyy/mafuyu/libs/grpc => ../../libs/grpc

require (
	github.com/dehwyy/mafuyu/libs/config v0.0.0-00010101000000-000000000000
	github.com/dehwyy/mafuyu/libs/grpc v0.0.0-00010101000000-000000000000
	github.com/dehwyy/mafuyu/libs/logger v0.0.0-00010101000000-000000000000
	github.com/dehwyy/mafuyu/libs/nats v0.0.0-00010101000000-000000000000
	github.com/getsentry/sentry-go v0.27.0
	github.com/nats-io/nats.go v1.33.1
	google.golang.org/grpc v1.61.1
	google.golang.org/protobuf v1.32.0
)

require (
	github.com/golang/protobuf v1.5.3 // indirect
	github.com/joho/godotenv v1.5.1 // indirect
	github.com/kelseyhightower/envconfig v1.4.0 // indirect
	github.com/klauspost/compress v1.17.5 // indirect
	github.com/mattn/go-colorable v0.1.13 // indirect
	github.com/mattn/go-isatty v0.0.20 // indirect
	github.com/nats-io/nkeys v0.4.7 // indirect
	github.com/nats-io/nuid v1.0.1 // indirect
	github.com/pkg/errors v0.9.1 // indirect
	github.com/rs/zerolog v1.32.0 // indirect
	golang.org/x/crypto v0.18.0 // indirect
	golang.org/x/net v0.18.0 // indirect
	golang.org/x/sys v0.17.0 // indirect
	golang.org/x/text v0.14.0 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20231106174013-bbf56f31fb17 // indirect
)
