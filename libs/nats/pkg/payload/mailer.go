package payload

type SendEmailPayload struct {
	To      string `json:"to"`
	Subject string `json:"subject"`
	Content []byte `json:"content"`
}

const SendEmailSubject = "mailer.do.send_email"
