package grpc

import (
	mafuyu_config "github.com/dehwyy/mafuyu/libs/config/go"
	"github.com/dehwyy/mafuyu/libs/grpc/gen/api"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

type RpcClients struct {
	AuthClient        api.AuthRpcClient
	IntegrationClient api.IntegrationsRpcClient
	OAuth2Client      api.OAuth2RpcClient
	PassportClient    api.PassportRpcClient
	TokensClient      api.TokensRpcClient
	UserClient        api.UserRpcClient
	CdnClient         api.CDNRpcClient
}

func NewRpcClients() (*RpcClients, func() error) {
	hosts := mafuyu_config.NewHosts()
	insec := grpc.WithTransportCredentials(insecure.NewCredentials())
	connErrors := make([]error, 10)

	authConn, err := grpc.Dial(hosts.Auth, insec)
	connErrors = append(connErrors, err)

	integrationsConn, err := grpc.Dial(hosts.Integrations, insec)
	connErrors = append(connErrors, err)

	oauth2Conn, err := grpc.Dial(hosts.Oauth2, insec)
	connErrors = append(connErrors, err)

	passportConn, err := grpc.Dial(hosts.Passport, insec)
	connErrors = append(connErrors, err)

	tokensConn, err := grpc.Dial(hosts.Tokens, insec)
	connErrors = append(connErrors, err)

	userConn, err := grpc.Dial(hosts.User, insec)
	connErrors = append(connErrors, err)

	cdnConn, err := grpc.Dial(hosts.CdnRpc, insec)
	connErrors = append(connErrors, err)

	for _, err = range connErrors {
		if err != nil {
			panic(err)
		}
	}

	return &RpcClients{
			AuthClient:        api.NewAuthRpcClient(authConn),
			IntegrationClient: api.NewIntegrationsRpcClient(integrationsConn),
			OAuth2Client:      api.NewOAuth2RpcClient(oauth2Conn),
			PassportClient:    api.NewPassportRpcClient(passportConn),
			TokensClient:      api.NewTokensRpcClient(tokensConn),
			UserClient:        api.NewUserRpcClient(userConn),
			CdnClient:         api.NewCDNRpcClient(cdnConn),
		}, func() error {

			for _, connection := range [7]*grpc.ClientConn{authConn, integrationsConn, oauth2Conn, passportConn, tokensConn, userConn, cdnConn} {
				err := connection.Close()
				if err != nil {
					return err
				}
			}

			return nil
		}
}
