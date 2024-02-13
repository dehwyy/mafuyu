type Path = string
interface PathValue {
  placeholder: string
}

interface Args {
  base_route: string
  current_route: string
  navigations: Record<Path, PathValue>
}

function UseSyncedNavigation({ base_route, current_route, navigations }: Args) {
  const get_current_value = () => {
    if (current_route.startsWith(base_route)) {
      const dyn = current_route.slice(base_route.length)
      return Object.keys(navigations).includes(dyn) ? dyn : "/"
    }

    return "/"
  }

  return {
    current_value: get_current_value(),
    iter: () => {
      return Object.keys(navigations).map(key => ({
        value: key,
        placeholder: navigations[key].placeholder,
      }))
    },
  }
}

export default UseSyncedNavigation
