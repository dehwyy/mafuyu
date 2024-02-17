package templates

import (
	"bytes"
	"fmt"
	"html/template"
)

type templateName = string

const (
	sendConfirmationEmail templateName = "send_confirmation_email.html"
)

func parseTemplate(filename templateName, data interface{}) ([]byte, error) {
	t, err := template.ParseFiles(fmt.Sprintf("templates/mailer.%s", filename))
	if err != nil {
		return nil, err
	}

	buf := new(bytes.Buffer)
	if err = t.Execute(buf, data); err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}

func NewSendConfirmationEmailTemplate(to, subject, code string) ([]byte, error) {
	return parseTemplate(sendConfirmationEmail, map[string]string{
		"To":      to,
		"Subject": subject,
		"Code":    code,
	})
}
