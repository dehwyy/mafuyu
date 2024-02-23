package logger

import (
	"github.com/getsentry/sentry-go"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/pkgerrors"
	"io"
	"os"
	"slices"
	"time"
)

type Logger = zerolog.Logger

type logHook struct {
	sentryClient *sentry.Client
}

func (h *logHook) Run(e *zerolog.Event, level zerolog.Level, msg string) {
	// It's ok for `sentryClient` to be nil
	if slices.Contains([]zerolog.Level{zerolog.FatalLevel}, level) && h.sentryClient != nil {
		defer h.sentryClient.Flush(5)
		e.Msg("Sent to Sentry.")

		var sentryLevel sentry.Level
		if level == zerolog.ErrorLevel {
			sentryLevel = sentry.LevelError
		} else {
			sentryLevel = sentry.LevelFatal
		}

		event := h.sentryClient.EventFromMessage(msg, sentryLevel)
		h.sentryClient.CaptureEvent(event, &sentry.EventHint{
			Data: map[string]any{
				"Stack": e.Stack(),
			},
			Context: e.GetCtx(),
		}, nil)
	}
}

func New(sentryClient *sentry.Client, production ...bool) zerolog.Logger {
	var out io.Writer

	if production == nil {
		production = []bool{false}
	}

	// Default STDOUT
	if production[0] {
		out = os.Stdout
	} else {
		// Pretty logging
		out = zerolog.ConsoleWriter{
			Out:        os.Stdout,
			TimeFormat: time.RFC822,
		}
	}

	zerolog.ErrorStackMarshaler = pkgerrors.MarshalStack
	zerolog.ErrorStackFieldName = "stack"

	logger := zerolog.New(out).With().Caller().Timestamp().Logger()

	logger.Hook(&logHook{sentryClient})
	return logger
}
