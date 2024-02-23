package main

import (
	"github.com/dehwyy/mafuyu/apps/mailer/service"
	mafuyuConfig "github.com/dehwyy/mafuyu/libs/config/go"
	rpc "github.com/dehwyy/mafuyu/libs/grpc/gen/api"
	"github.com/dehwyy/mafuyu/libs/logger"
	mafuyuSentry "github.com/dehwyy/mafuyu/libs/sentry"
	"github.com/getsentry/sentry-go"
	"github.com/nats-io/nats.go"
	"google.golang.org/grpc"
	"net"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	config := mafuyuConfig.NewConfig()
	hosts := mafuyuConfig.NewHosts()

	sentryClient := mafuyuSentry.NewSentryClient(config)
	l := logger.New(sentryClient, config.IsProduction)

	lis, _ := net.Listen("tcp", hosts.MailerRpc)
	conn, _ := nats.Connect(mafuyuConfig.NatsMailerServer)
	encodedConn, err := nats.NewEncodedConn(conn, nats.JSON_ENCODER)
	if err != nil {
		panic(err)
	}

	defer conn.Close()
	defer encodedConn.Close()
	srv := service.NewMailerService(encodedConn)
	grpcServer := grpc.NewServer()
	rpc.RegisterMailerRpcServer(grpcServer, srv)

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)

	go func() {
		defer sentry.Recover()

		l.Info().Msgf("mailer server listening at %v", lis.Addr().String())
		if err := grpcServer.Serve(lis); err != nil {
			l.Fatal().Err(err).Msg("failed to serve")
		}
	}()

	<-c
}
