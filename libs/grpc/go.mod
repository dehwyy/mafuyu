module github.com/dehwyy/mafuyu/libs/grpc

go 1.21.2

replace github.com/dehwyy/mafuyu/libs/config => ../config

require (
	github.com/dehwyy/mafuyu/libs/config v0.0.0-00010101000000-000000000000
	github.com/twitchtv/twirp v8.1.3+incompatible
	google.golang.org/grpc v1.61.1
	google.golang.org/protobuf v1.32.0
)

require (
	github.com/golang/protobuf v1.5.3 // indirect
	github.com/joho/godotenv v1.5.1 // indirect
	github.com/kelseyhightower/envconfig v1.4.0 // indirect
	github.com/pkg/errors v0.9.1 // indirect
	github.com/stretchr/testify v1.8.4 // indirect
	golang.org/x/net v0.18.0 // indirect
	golang.org/x/sys v0.17.0 // indirect
	golang.org/x/text v0.14.0 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20231106174013-bbf56f31fb17 // indirect
)
