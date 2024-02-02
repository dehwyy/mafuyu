package makoto_db

import (
	"fmt"

	"github.com/dehwyy/makoto/libs/db/src/models"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func New(database_url string) *gorm.DB {
	fmt.Printf("database_url: %s\n", database_url)

	if database_url == "" {
		database_url = "host=localhost user=postgres password=postgres dbname=postgres port=5432 sslmode=disable"
	}

	db, err := gorm.Open(postgres.New(postgres.Config{
		DSN: database_url,
	}), &gorm.Config{})

	if err != nil {
		panic(err)
	}

	db_settings, err := db.DB()
	if err != nil {
		panic(err)
	}

	db_settings.SetMaxOpenConns(100)
	db_settings.SetMaxIdleConns(10)
	db_settings.SetConnMaxIdleTime(5)

	err = db.AutoMigrate(
		models.Language{},
		models.Post{}, models.PostReports{}, models.Comment{},
		models.UserCredentials{}, models.UserTokens{}, models.UserIntegrations{}, models.User{},
	)

	if err != nil {
		panic(err)
	}

	return db
}
