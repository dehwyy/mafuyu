package main

import (
	"context"
	"github.com/dehwyy/mafuyu/apps/mailer-async/internal"
	"github.com/dehwyy/mafuyu/apps/mailer-async/internal/service"
	mafuyuConfig "github.com/dehwyy/mafuyu/libs/config/go"
	"github.com/dehwyy/mafuyu/libs/logger"
	mafuyuSentry "github.com/dehwyy/mafuyu/libs/sentry"
	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
	"time"
)

func main() {
	cfg := mafuyuConfig.NewConfig()
	sentryClient := mafuyuSentry.NewSentryClient(cfg)

	l := logger.New(sentryClient, cfg.IsProduction)

	client, _ := nats.Connect(nats.DefaultURL)
	defer client.Close()

	js, _ := jetstream.New(client)
	stream, _ := js.CreateOrUpdateStream(context.Background(), jetstream.StreamConfig{
		Name:        "mailer",
		Description: "Mailer microservice message-broker using NATS",
		Subjects:    []string{"mailer.do.>"},
		MaxAge:      10 * time.Minute,
		Retention:   jetstream.WorkQueuePolicy,
	})

	consumer, _ := stream.CreateOrUpdateConsumer(context.Background(), jetstream.ConsumerConfig{
		Name: "mailer",
	})

	gmailEmailService := service.NewGmailEmailService(cfg.GmailSenderName, cfg.GmailSenderAddr, cfg.GmailSenderPassword)
	router := internal.NewRouter(gmailEmailService)

	messagesIter, _ := consumer.Messages()
	defer messagesIter.Stop()

	l.Info().Msgf("Listening on: %s", nats.DefaultURL)

	for {
		msg, err := messagesIter.Next()
		if err != nil {
			l.Fatal().Err(err)
		}

		l.Trace().Msgf("received message: %s", msg.Subject())
		err = router.Handle(msg)
		if err != nil {
			l.Error().Err(err)
		}
	}
}
