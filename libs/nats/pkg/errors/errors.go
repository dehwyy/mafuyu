package errors

type NatsError struct {
	msg string
}

func (e NatsError) Error() string {
	return e.msg
}

func NewNatsError(msg string) *NatsError {
	return &NatsError{msg: msg}
}
