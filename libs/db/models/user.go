package models

import (
	"time"

	"github.com/dehwyy/makoto/libs/db/models/custom_dt"

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
	Languages       []*Language `gorm:"many2many:user_languages;foreignKey:user_id"`
	Friends         []*User     `gorm:"many2many:user_friends;foreignKey:user_id;joinForeignKey:user_id;references:user_id;joinReferences:friend_user_id"`
	Blocked         []*User     `gorm:"many2many:user_blocked;foreignKey:user_id;joinForeignKey:user_id;references:user_id;joinReferences:blocked_user_id"`
	Followers       []*User     `gorm:"many2many:user_followers;foreignKey:user_id;joinForeignKey:user_id;references:user_id;joinReferences:followed_to_user_id"`
}

type UserCredentials struct {
	ID         uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"` // @see https://www.postgresql.org/docs/current/functions-uuid
	ProviderId string
	Username   string             `gorm:"not null;uniqueIndex"`
	Email      string             `gorm:"unique"`
	Password   string             // hashed password
	Role       custom_dt.UserRole `gorm:"default:2;not null"`
	CreatedAt  time.Time

	// relations
	Tokens      UserTokens       `gorm:"foreignKey:UserId;references:ID"`
	Integration UserIntegrations `gorm:"foreignKey:UserId;references:ID"`
	User        User             `gorm:"foreignKey:UserId;references:ID"`
}

type UserTokens struct {
	ID           uuid.UUID      `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId       uuid.UUID      `gorm:"not null;index"` // foreign key
	AccessTokens pq.StringArray `gorm:"type:text[]"`
	Provider     custom_dt.AuthProvider
	RefreshToken string // nullable (as some oauth2 apps doesn't provide refresh_token (for example, GitHub)
}

type UserIntegrations struct {
	ID                uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId            uuid.UUID `gorm:"not null;index"`
	UserIntegrationId string    `gorm:"not_null"`
	IntegrationName   string    `gorm:"not null"`
}
