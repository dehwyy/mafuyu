module github.com/dehwyy/makoto/libs/db

go 1.21.2

replace (
	github.com/dehwyy/makoto/libs/config => ../config
	github.com/dehwyy/makoto/libs/logger => ../logger
)

require (
	github.com/dehwyy/makoto/libs/config v0.0.0-00010101000000-000000000000
	github.com/google/uuid v1.5.0
	github.com/lib/pq v1.10.9
	gorm.io/driver/postgres v1.5.4
	gorm.io/gorm v1.25.5
)

require (
	github.com/jackc/pgpassfile v1.0.0 // indirect
	github.com/jackc/pgservicefile v0.0.0-20221227161230-091c0ba34f0a // indirect
	github.com/jackc/pgx/v5 v5.4.3 // indirect
	github.com/jinzhu/inflection v1.0.0 // indirect
	github.com/jinzhu/now v1.1.5 // indirect
	github.com/joho/godotenv v1.5.1 // indirect
	github.com/kelseyhightower/envconfig v1.4.0 // indirect
	github.com/stretchr/testify v1.8.4 // indirect
	golang.org/x/crypto v0.18.0 // indirect
	golang.org/x/text v0.14.0 // indirect
)
