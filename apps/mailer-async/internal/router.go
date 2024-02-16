package internal

import (
	mafuyuNats "github.com/dehwyy/mafuyu/libs/nats/pkg"
	mafuyuNatsError "github.com/dehwyy/mafuyu/libs/nats/pkg/errors"
	mafuyuNatsPayload "github.com/dehwyy/mafuyu/libs/nats/pkg/payload"
	"github.com/nats-io/nats.go/jetstream"
)

type MailService interface {
	SendEmail(to []string, subject string, content []byte) error
}

type Router struct {
	mailService MailService
}

func NewRouter(mailService MailService) *Router {
	return &Router{
		mailService: mailService,
	}
}

func (r *Router) Handle(msg jetstream.Msg) error {
	if err := msg.Ack(); err != nil {
		return err
	}

	tools := mafuyuNats.NewTools(msg)
	subject, err := tools.GetSubject()
	if err != nil {
		return err
	}

	switch subject {
	case "send_email":
		return r.SendMail(msg)
	default:
		return mafuyuNatsError.NewNatsError("subject not found")
	}
}

func (r *Router) SendMail(msg jetstream.Msg) error {
	payload, err := mafuyuNats.GetPayload[mafuyuNatsPayload.SendEmailPayload](msg.Data())
	if err != nil {
		return err
	}

	return r.mailService.SendEmail([]string{payload.To}, payload.Subject, payload.Content)
}
