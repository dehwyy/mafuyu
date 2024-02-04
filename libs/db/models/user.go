package models

import (
	"github.com/dehwyy/makoto/libs/db/src/models/custom_dt"
	"time"

	"github.com/google/uuid"
	"github.com/lib/pq"
)

type User struct {
	ID              uuid.UUID        `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId          uuid.UUID        `gorm:"not null;uniqueIndex"`
	UserCredentials *UserCredentials `gorm:"foreignKey:UserId"`
	Location        string
	Birthday        time.Time
	Pseudonym       string
	Bio             string
	Picture         string
	Languages       []*Language `gorm:"many2many:user_languages;"`
	Friends         []*User     `gorm:"many2many:user_friends;"`
	Blocked         []*User     `gorm:"many2many:user_blocked"`
	Followers       []*User     `gorm:"many2many:user_followers;"`
	Posts           []Post      `gorm:"foreignKey:AuthorId"`
	LikedPosts      []*Post     `gorm:"many2many:user_liked_posts;"`
	HiddenPosts     []*Post     `gorm:"many2many:user_hidden_posts;"`
	Comments        []Comment   `gorm:"foreignKey:AuthorId"`
	LikedComments   []*Comment  `gorm:"many2many:user_liked_comments;"`
}

type UserCredentials struct {
	ID         uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"` // @see https://www.postgresql.org/docs/current/functions-uuid
	ProviderId string
	Username   string `gorm:"not null;uniqueIndex"`
	Email      string `gorm:"unique"`
	Password   string // hashed password
	CreatedAt  time.Time

	// relations
	Tokens      UserTokens       `gorm:"foreignKey:UserId;references:ID"`
	Integration UserIntegrations `gorm:"foreignKey:UserId;references:ID"`
	User        User             `gorm:"foreignKey:UserId;references:ID"`
}

type UserTokens struct {
	ID           uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId       uuid.UUID `gorm:"not null;index"` // foreign key
	Provider     custom_dt.AuthProvider
	AccessTokens pq.StringArray `gorm:"type:text[]"`
	RefreshToken string         // nullable (as some oauth2 apps doesn't provide refresh_token (for example, GitHub)
}

type UserIntegrations struct {
	ID                uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId            uuid.UUID `gorm:"not null;index"`
	UserIntegrationId string    `gorm:"not_null"`
	IntegrationName   string    `gorm:"not null"`
}
