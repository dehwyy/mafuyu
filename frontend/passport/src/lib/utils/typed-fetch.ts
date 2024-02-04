class TypedFetch<Req, Res> {
  private RouteBase = "/api/v1"

  constructor(
    private route: string,
    private method: string,
  ) {}

  async fetch(req: Req): Promise<Res> {
    const response = await fetch(this.api_route, {
      method: this.method,
      body: JSON.stringify(req),
    })

    return response.json()
  }

  async get_request_with_response_creator(req: Request): Promise<[Req, typeof this.create_new_response]> {
    const r = await req.json()
    return [r, this.create_new_response]
  }

  private create_new_response(res: Res, response_init?: ResponseInit): Response {
    return new Response(JSON.stringify(res), response_init)
  }

  private get api_route(): string {
    return `${this.RouteBase}/${this.route}`
  }
}

export const Routes = {
  "user/edit": new TypedFetch<
    {
      userId: string
      pseudonym: string
      image: string
    },
    {}
  >("user/edit", "POST"),
  oauth: new TypedFetch<
    {
      provider: string
    },
    { redirect_url: string }
  >("oauth", "POST"),
} as const
