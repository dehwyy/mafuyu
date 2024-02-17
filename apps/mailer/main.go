package main

import (
	"github.com/dehwyy/mafuyu/apps/mailer/service"
	mafuyuConfig "github.com/dehwyy/mafuyu/libs/config/go"
	rpc "github.com/dehwyy/mafuyu/libs/grpc/gen/api"
	mafuyuLogger "github.com/dehwyy/mafuyu/libs/logger/src"
	"github.com/nats-io/nats.go"
	"google.golang.org/grpc"
	"net"
)

func main() {
	l := mafuyuLogger.New()
	//cfg := mafuyuConfig.NewConfig()
	hosts := mafuyuConfig.NewHosts()

	lis, err := net.Listen("tcp", hosts.MailerRpc)
	if err != nil {
		panic(err)
	}

	conn, _ := nats.Connect(mafuyuConfig.NatsMailerServer)
	encodedConn, err := nats.NewEncodedConn(conn, nats.JSON_ENCODER)
	if err != nil {
		panic(err)
	}
	defer encodedConn.Close()

	srv := service.NewMailerService(encodedConn)

	rpcServer := grpc.NewServer()
	rpc.RegisterMailerRpcServer(rpcServer, srv)

	l.Infof("mailer server listening at %v", lis.Addr())
	if err := rpcServer.Serve(lis); err != nil {
		l.Fatalf("failed to serve: %v", err)
	}
}
