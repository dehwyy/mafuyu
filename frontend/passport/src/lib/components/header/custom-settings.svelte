<script lang="ts">
  import SettingsIconRaw from "$lib/assets/gear.svg?raw"

  import { MinimalisticHeader, AnimatedBackground, RgbCard, ThemeSelector } from "./custom-settings"

  import { Icon, Button } from "$lib/components/header/nav-items"

  import { useUserInfo } from "$lib/query/user"
  import { authedUserStore } from "$lib/stores/user"
  import { type PopupSettings, popup } from "@skeletonlabs/skeleton"

  const customSettingsSelectClick: PopupSettings = {
    target: "custom-settings-popup",
    placement: "left",
    event: "click",
  }

  const [user, userStore] = useUserInfo({ oneofKind: "userId", userId: $authedUserStore?.id })
  $: userStore.set({ getBy: { oneofKind: "userId", userId: $authedUserStore?.id } })

  // $: username = $authedUserStore?.username || ($user?.data?.username as string)

  $: panels = [{ component: ThemeSelector }, { component: AnimatedBackground }, { component: RgbCard }, { component: MinimalisticHeader }]
</script>

<div use:popup={customSettingsSelectClick} class="cursor-pointer flex-auto">
  <Button closeAfterClick={false}>
    <Icon rawIcon={SettingsIconRaw} />
    <p>Settings</p>
  </Button>
</div>
<div data-popup="custom-settings-popup" class="pr-1">
  <div class="card shadow-md shadow-primary-500 rounded-2xl min-w-[200px] py-4 px-1">
    <ul class="flex flex-col gap-y-1">
      {#each panels as panel}
        <li>
          <svelte:component this={panel.component} />
        </li>
      {/each}
    </ul>
  </div>
</div>
