package main

import (
	"os"

	makoto_config "github.com/dehwyy/makoto/libs/config/go"
	makoto_db "github.com/dehwyy/makoto/libs/db/go"
)

func main() {

	command := os.Args[1]

	if command == "migrate" {
		makoto_db.New(makoto_config.NewConfig().DatabaseDsn)
	}
}
