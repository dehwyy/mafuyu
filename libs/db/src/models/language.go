package models

type Language struct {
	ID    uint    `gorm:"primary_key"`
	Name  string  `gorm:"not null"`
	Users []*User `gorm:"many2many:user_languages;"`
}
