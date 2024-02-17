package service

import (
	"context"
	"github.com/dehwyy/mafuyu/apps/mailer/templates"
	rpc "github.com/dehwyy/mafuyu/libs/grpc/gen/api"
	mafuyuNatsPayload "github.com/dehwyy/mafuyu/libs/nats/pkg/payload"
	"github.com/nats-io/nats.go"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type MailerService struct {
	natsConn *nats.EncodedConn
	rpc.UnimplementedMailerRpcServer
}

func NewMailerService(natsConn *nats.EncodedConn) *MailerService {
	return &MailerService{
		natsConn: natsConn,
	}
}

func (s *MailerService) SendConfirmationEmail(_ context.Context, req *rpc.SendConfirmationEmailRequest) (*emptypb.Empty, error) {

	parsedTemplateBytes, err := templates.NewSendConfirmationEmailTemplate(req.To, req.Subject, req.Code)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to parse template %v", err)
	}

	err = s.natsConn.Publish(mafuyuNatsPayload.SendEmailSubject, &mafuyuNatsPayload.SendEmailPayload{
		To:      req.To,
		Subject: req.Subject,
		Content: parsedTemplateBytes},
	)

	return &emptypb.Empty{}, err
}
