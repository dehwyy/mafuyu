package models

import (
	"github.com/dehwyy/makoto/libs/db/src/models/custom_dt"
	"time"

	"github.com/google/uuid"
	"github.com/lib/pq"
)

type UserCredentials struct {
	ID        uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"` // @see https://www.postgresql.org/docs/current/functions-uuid
	Username  string    `gorm:"unique;not null;index"`
	Email     string    `gorm:"unique	"`
	Password  string    // hashed password
	CreatedAt time.Time

	// relations
	Tokens      UserTokens        `gorm:"foreignKey:UserId;references:ID"`
	Integration []UserIntegration `gorm:"foreignKey:UserId;references:ID"`
}

type UserTokens struct {
	ID           uuid.UUID              `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId       uuid.UUID              `gorm:"not null;index"` // foreign key
	Provider     custom_dt.AuthProvider `gorm:"not null"`
	AccessToken  pq.StringArray         `gorm:"type:text[]"`
	RefreshToken string                 // nullable (as some oauth2 apps doesn't provide refresh_token (for example, GitHub)
}

type UserIntegration struct {
	ID                uuid.UUID `gorm:"type:uuid;primary_key;default:gen_random_uuid()"`
	UserId            uuid.UUID `gorm:"not null;index"`
	UserIntegrationId string    `gorm:"not_null"`
	IntegrationName   string    `gorm:"not null"`
}
