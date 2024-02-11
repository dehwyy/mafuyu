package _go

import (
	"fmt"
	"github.com/dehwyy/makoto/libs/db/models"
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

	var language_record_count int64
	err = db.Model(models.Language{}).Count(&language_record_count).Error
	if err != nil {
		panic(err)
	} else if language_record_count == 0 {
		fmt.Println("insert default languages...")

		languages := []models.Language{
			{ID: 1, Name: "arabic"},
			{ID: 2, Name: "dutch"},
			{ID: 3, Name: "english"},
			{ID: 4, Name: "french"},
			{ID: 5, Name: "german"},
			{ID: 6, Name: "hindi"},
			{ID: 7, Name: "indonesian"},
			{ID: 8, Name: "italian"},
			{ID: 9, Name: "japanese"},
			{ID: 10, Name: "korean"},
			{ID: 11, Name: "chinese"},
			{ID: 12, Name: "polish"},
			{ID: 13, Name: "portuguese"},
			{ID: 14, Name: "russian"},
			{ID: 15, Name: "spanish"},
			{ID: 16, Name: "thai"},
			{ID: 17, Name: "turkish"},
		}

		err = db.CreateInBatches(languages, len(languages)).Error
		if err != nil {
			panic(err)
		}

		fmt.Println("insert default languages... done!")
	}

	if err != nil {
		panic(err)
	}

	return db
}
