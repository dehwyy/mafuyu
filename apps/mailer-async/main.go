package main

import (
	"context"
	"github.com/dehwyy/mafuyu/apps/mailer-async/internal"
	"github.com/dehwyy/mafuyu/apps/mailer-async/internal/service"
	mafuyuConfig "github.com/dehwyy/mafuyu/libs/config/go"
	mafuyuLogger "github.com/dehwyy/mafuyu/libs/logger/src"
	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
	"time"
)

func main() {
	l := mafuyuLogger.New()
	cfg := mafuyuConfig.NewConfig()

	client, err := nats.Connect(nats.DefaultURL)
	if err != nil {
		l.Fatalf("connect to nats %v", err)
	}
	defer client.Close()

	js, err := jetstream.New(client)
	if err != nil {
		l.Fatalf("connect to jetstream %v", err)
	}

	stream, err := js.CreateOrUpdateStream(context.Background(), jetstream.StreamConfig{
		Name:        "mailer",
		Description: "Mailer microservice message-broker using NATS",
		Subjects:    []string{"mailer.do.>"},
		MaxAge:      10 * time.Minute,
		Retention:   jetstream.WorkQueuePolicy,
	})
	if err != nil {
		l.Fatalf("create stream %v", err)
	}

	consumer, err := stream.CreateOrUpdateConsumer(context.Background(), jetstream.ConsumerConfig{
		Name: "mailer",
	})
	if err != nil {
		l.Fatalf("create consumer %v", err)
	}

	gmailEmailService := service.NewGmailEmailService(cfg.GmailSenderName, cfg.GmailSenderAddr, cfg.GmailSenderPassword)
	router := internal.NewRouter(gmailEmailService)

	messagesIter, err := consumer.Messages()
	if err != nil {
		l.Fatalf("get messages %v", err)
	}

	defer messagesIter.Stop()

	l.Infof("Listening on: %s", nats.DefaultURL)

	for {
		msg, err := messagesIter.Next()
		if err != nil {
			l.Fatalf("get message %v", err)
		}

		l.Infof("received message: %s", msg.Subject())
		err = router.Handle(msg)
		if err != nil {
			l.Errorf("router: %v", err)
		}
	}
}
