package custom_dt

import (
	"database/sql/driver"
	"errors"
)

type AuthProvider string

const (
	native AuthProvider = "native" // via common credentials
	github AuthProvider = "github"
	google AuthProvider = "google"
)

func (provider *AuthProvider) Scan(value interface{}) error {
	s, ok := value.(AuthProvider)
	if !ok {
		return errors.New("failed to parse provider_name to string")
	}

	*provider = s
	return nil
}

func (provider *AuthProvider) Value() (driver.Value, error) {
	return provider, nil
}
