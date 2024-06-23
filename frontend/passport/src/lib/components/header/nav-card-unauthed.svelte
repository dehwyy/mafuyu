<script lang="ts">
  import ThemeSelectIconRaw from '$lib/assets/theme-select.svg?raw'
  import LoginIconRaw from '$lib/assets/user.svg?raw'
  import { Button, Icon } from '$lib/components/header/nav-items'
  import { Routes } from '$lib/const'

  import ThemeSelector from './custom-settings/theme-selector.svelte'
</script>

<ul class="mt-5">
  <li>
    <a href={Routes.Login}>
      <Button>
        <Icon rawIcon={LoginIconRaw} />
        <span>Log in</span>
      </Button>
    </a>
  </li>
  <li>
    <div>
      <ThemeSelector>
        <span class="icon-sm block">{@html ThemeSelectIconRaw}</span>
      </ThemeSelector>
    </div>
  </li>
</ul>
