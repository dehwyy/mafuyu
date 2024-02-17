package _go

import (
	"fmt"
	"github.com/kelseyhightower/envconfig"
)

type Hosts struct {
	Gateway      string `required:"true"    envconfig:"GATEWAY"`
	TwirpGateway string `required:"true"    envconfig:"TWIRP_GATEWAY"`
	Auth         string `required:"true"    envconfig:"AUTH"`
	Oauth2       string `required:"true"    envconfig:"OAUTH2"`
	Tokens       string `required:"true"    envconfig:"TOKENS"`
	Passport     string `required:"true"    envconfig:"PASSPORT"`
	User         string `required:"true"    envconfig:"USER_RPC"`
	Integrations string `required:"true"    envconfig:"INTEGRATIONS"`
	CdnRpc       string `required:"true"    envconfig:"CDN_RPC"`
	MailerRpc    string `required:"true"    envconfig:"MAILER_RPC"`
}

func NewHosts() *Hosts {
	initEnv()

	var hosts Hosts

	if err := envconfig.Process("", &hosts); err != nil {
		fmt.Printf("failed to process env: %v\n", err)
		panic(err)
	}

	return &hosts
}
