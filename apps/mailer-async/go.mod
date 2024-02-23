module github.com/dehwyy/mafuyu/apps/mailer-async

go 1.21.2

replace github.com/dehwyy/mafuyu/libs/nats => ../../libs/nats

replace github.com/dehwyy/mafuyu/libs/logger => ../../libs/logger

replace github.com/dehwyy/mafuyu/libs/config => ../../libs/config
replace github.com/dehwyy/mafuyu/libs/sentry => ../../libs/sentry

require (
	github.com/jordan-wright/email v4.0.1-0.20210109023952-943e75fe5223+incompatible
	github.com/nats-io/nats.go v1.33.1
)

require (
	github.com/klauspost/compress v1.17.5 // indirect
	github.com/nats-io/nkeys v0.4.7 // indirect
	github.com/nats-io/nuid v1.0.1 // indirect
	golang.org/x/crypto v0.18.0 // indirect
	golang.org/x/sys v0.17.0 // indirect
	golang.org/x/text v0.14.0 // indirect
)
