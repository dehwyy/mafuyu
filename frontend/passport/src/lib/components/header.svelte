<script lang="ts">
	import { AppBar, type PopupSettings, popup, LightSwitch } from '@skeletonlabs/skeleton';
// @ts-ignore
import Options from "svelte-icons/fa/FaBars.svelte"
// @ts-ignore
import UserIcon from 'svelte-icons/fa/FaRegUser.svelte'
// @ts-ignore
import MoonIcon from 'svelte-icons/fa/FaRegMoon.svelte'
// @ts-ignore
import ThemeSelectIcon from 'svelte-icons/di/DiNetbeans.svelte'

import ThemeSelector from '$lib/components/theme-selector.svelte'

const settingsClick: PopupSettings = {
	event: "click",
	placement: "bottom-start",
	target: "settings-popup"
}

</script>

<AppBar>
	<svelte:fragment slot="lead">
		<div class="icon ml-[20px] cursor-pointer select-none" use:popup={settingsClick}>
			<Options />
		</div>
		<div data-popup="settings-popup" class="card p-6 mt-4" >
			<div class="menu-item">
				<div class="icon">
					<UserIcon />
				</div>
				<p>dehwyy</p>
			</div>
			<hr class="my-4"/>
			<div class="menu-item">
				<div class="icon">
					<MoonIcon />
				</div>
				<LightSwitch />
			</div>
			<hr class="my-4"/>
			<div class="menu-item">
				<div class="icon">
					<ThemeSelectIcon />
				</div>
				<ThemeSelector />
			</div>
		</div>
	</svelte:fragment>
	<h3 class="h3 ml-3">
		❄️Mafuyu
	</h3>
<!--	<svelte:fragment slot="trail">(actions)</svelte:fragment>-->
</AppBar>

<style>
	.menu-item {
		@apply flex gap-x-5 h-[20px] items-center;
	}

	.icon {
			width: 20px;
			height: 20px;
	}
</style>