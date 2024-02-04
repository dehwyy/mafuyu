package _go

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

	var language_record_count int64
	err = db.Model(models.Language{}).Count(&language_record_count).Error
	if err != nil {
		panic(err)
	} else if language_record_count == 0 {
		fmt.Println("insert default languages...")

		languages := []models.Language{
			{ID: 1, Name: "Arabic"},
			{ID: 2, Name: "Dutch"},
			{ID: 3, Name: "English"},
			{ID: 4, Name: "French"},
			{ID: 5, Name: "German"},
			{ID: 6, Name: "Hindi"},
			{ID: 7, Name: "Indonesian"},
			{ID: 8, Name: "Italian"},
			{ID: 9, Name: "Japanese"},
			{ID: 10, Name: "Korean"},
			{ID: 11, Name: "Chinese"},
			{ID: 12, Name: "Polish"},
			{ID: 13, Name: "Portuguese"},
			{ID: 14, Name: "Russian"},
			{ID: 15, Name: "Spanish"},
			{ID: 16, Name: "Thai"},
			{ID: 17, Name: "Turkish"},
		}

		err = db.CreateInBatches(languages, len(languages)).Error
		if err != nil {
			panic(err)
		}

	}

	if err != nil {
		panic(err)
	}

	return db
}
