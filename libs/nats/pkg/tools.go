package pkg

import (
	"encoding/json"
	"github.com/dehwyy/mafuyu/libs/nats/pkg/errors"
	"github.com/nats-io/nats.go/jetstream"
	"strings"
)

type Tools struct {
	Msg jetstream.Msg
}

func NewTools(msg jetstream.Msg) *Tools {
	return &Tools{Msg: msg}
}

func (t *Tools) GetSubject() (string, error) {
	parts := strings.SplitN(t.Msg.Subject(), ".", 3)
	if len(parts) < 3 {
		return "", errors.NewNatsError("malformed subject")
	}

	v := parts[2]
	return v, nil
}

func GetPayload[T any](b []byte) (T, error) {
	var v T
	if err := json.Unmarshal(b, &v); err != nil {
		return v, err
	}

	return v, nil
}
