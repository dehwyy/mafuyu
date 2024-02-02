package models

import (
	"github.com/google/uuid"
	"github.com/lib/pq"
	"time"
)

type Post struct {
	ID        uint `gorm:"primary_key"`
	CreatedAt time.Time
	UpdatedAt time.Time
	AuthorId  uuid.UUID      `gorm:"not null;uniqueIndex"`
	Author    *User          `gorm:"foreignKey:AuthorId"`
	Images    pq.StringArray `gorm:"type:text[]"`
	Views     *uint          `gorm:"default:0"`
	Likes     []*User        `gorm:"many2many:user_liked_posts"`
	HiddenFor []*User        `gorm:"many2many:user_hidden_posts"`
	Reports   []PostReports
}

type PostReports struct {
	ID      uint `gorm:"primary_key"`
	PostId  uint
	Kind    uint `gorm:"not null;"`
	Message string
}

type Comment struct {
	ID        uint `gorm:"primary_key"`
	CreatedAt time.Time
	UpdatedAt time.Time
	AuthorId  uuid.UUID      `gorm:"not null;uniqueIndex"`
	Author    *User          `gorm:"foreignKey:AuthorId"`
	Likes     []*User        `gorm:"many2many:user_liked_comments"`
	Images    pq.StringArray `gorm:"type:text[]"`
}
