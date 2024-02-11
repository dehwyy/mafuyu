package makoto_log

import (
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

type Logger interface {
	Debugf(format string, args ...interface{})
	Warnf(format string, args ...interface{})
	Errorf(format string, args ...interface{})
	Infof(format string, args ...interface{})
	Fatalf(format string, args ...interface{})
}

func New() Logger {
	logger_cfg := zap.Config{
		Encoding:    "json",
		Level:       zap.NewAtomicLevelAt(zap.DebugLevel),
		OutputPaths: []string{"stdout"},
		EncoderConfig: zapcore.EncoderConfig{
			// message
			MessageKey: "_MESSAGE_",
			// callstack
			CallerKey:    "_CALL FROM_",
			EncodeCaller: zapcore.ShortCallerEncoder,
			// time
			TimeKey:    "_TIME_",
			EncodeTime: zapcore.TimeEncoderOfLayout("__15:04:05__"),
			// name of the field already says what it is :)
			LineEnding: "\t__EOF__\n",
		},
	}

	logger, err := logger_cfg.Build()
	defer logger.Sync()

	if err != nil {
		panic(err)
	}

	surgared_logger := logger.Sugar()
	return surgared_logger
}
