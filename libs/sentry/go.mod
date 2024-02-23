module github.com/dehwyy/mafuyu/libs/sentry

go 1.21.5

replace github.com/dehwyy/mafuyu/libs/config => ../config

require (
	github.com/dehwyy/mafuyu/libs/config v0.0.0-00010101000000-000000000000
	github.com/getsentry/sentry-go v0.27.0
)

require (
	github.com/joho/godotenv v1.5.1 // indirect
	github.com/kelseyhightower/envconfig v1.4.0 // indirect
	golang.org/x/sys v0.6.0 // indirect
	golang.org/x/text v0.8.0 // indirect
)
