import type { Actions } from './$types'

export const actions = {
  create: async ({request, cookies, }) => {
    const data = await request.formData()

    const username = data.get('username')
    const email = data.get("email")
    const password = data.get('password')

    console.log(username, email, password)

    return {
      success: true
    }
  }
} satisfies Actions