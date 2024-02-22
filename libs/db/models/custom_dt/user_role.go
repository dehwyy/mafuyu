package custom_dt

import (
	"database/sql/driver"
	"errors"
)

type UserRole uint8

const (
	Unauthorized UserRole = 1
	User         UserRole = 2
	Admin        UserRole = 3
	CEO          UserRole = 4
)

func (role *UserRole) Scan(value interface{}) error {
	s, ok := value.(UserRole)
	if !ok {
		return errors.New("failed to parse role to int")
	}

	*role = s
	return nil
}

func (role *UserRole) Value() (driver.Value, error) {
	return *role, nil
}
