package main

import (
	"os"

	makoto_config "github.com/dehwyy/makoto/libs/config/src/go"
	makoto_db "github.com/dehwyy/makoto/libs/db/src/go"
)

func main() {

	command := os.Args[1]

	if command == "migrate" {
		makoto_db.New(makoto_config.NewConfig().DatabaseDsn)

	} else if command == "test-drop" {
		db := makoto_db.New(makoto_config.NewConfig().DatabaseTestDsn)
		db.Exec("DELETE FROM user_tokens")
		db.Exec("DELETE FROM user_oauths")
		db.Exec("DELETE FROM user_credentials")

	} else if command == "test-migrate" {
		makoto_db.New(makoto_config.NewConfig().DatabaseTestDsn)
	}

}
