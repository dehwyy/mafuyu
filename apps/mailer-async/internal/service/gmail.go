package service

import (
	"fmt"
	"github.com/jordan-wright/email"
	"net/smtp"
)

const (
	smtpAuthGmailAddr   = "smtp.gmail.com"
	smtpServerGmailAddr = "smtp.gmail.com:587"
)

type GmailEmailService struct {
	fromName     string
	fromAddr     string
	fromPassword string
}

func NewGmailEmailService(fromName string, fromAddr string, fromPassword string) *GmailEmailService {
	return &GmailEmailService{
		fromName:     fromName,
		fromAddr:     fromAddr,
		fromPassword: fromPassword,
	}
}

func (m *GmailEmailService) SendEmail(to []string, subject string, content []byte) error {
	mail := email.NewEmail()

	mail.From = fmt.Sprintf("%s <%s>", m.fromName, m.fromAddr)
	mail.Subject = subject
	mail.To = to
	mail.HTML = content

	smtpAuth := smtp.PlainAuth("", m.fromAddr, m.fromPassword, smtpAuthGmailAddr)
	return mail.Send(smtpServerGmailAddr, smtpAuth)
}
