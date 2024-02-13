import { useQuery } from "@sveltestack/svelte-query"

export const useUserInfo = (username: string) =>
  useQuery({
    queryKey: ["user_info", username],
    queryFn: async () => {
      return null
    },
  })
