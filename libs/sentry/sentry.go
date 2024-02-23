package mafuyuSentry

import (
	mafuyuConfig "github.com/dehwyy/mafuyu/libs/config/go"
	"github.com/getsentry/sentry-go"
	"strconv"
)

func NewSentryClient(config *mafuyuConfig.Config) *sentry.Client {
	if len(config.SentryDsn) == 0 {
		return nil
	}

	sentryOptions := sentry.ClientOptions{
		Dsn:                config.SentryDsn,
		EnableTracing:      true,
		ProfilesSampleRate: 1.0,
		TracesSampleRate:   1.0,
		SampleRate:         1,
		Tags: map[string]string{
			"Production": strconv.FormatBool(config.IsProduction),
		},
		Debug: !config.IsProduction,
	}

	sentryClient, err := sentry.NewClient(sentryOptions)
	if err != nil {
		panic(err)
	}

	err = sentry.Init(sentryOptions)
	if err != nil {
		panic(err)
	}

	return sentryClient
}
