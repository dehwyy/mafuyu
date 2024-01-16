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
// @ts-ignore
import LogoutIcon from 'svelte-icons/io/IoIosLogOut.svelte'

import ThemeSelector from '$lib/components/theme-selector.svelte'
import { user_store } from '$lib/stores/user';
import { Routes } from '$lib/const';

const settingsClick: PopupSettings = {
	event: "click",
	placement: "bottom-start",
	target: "settings-popup"
}

</script>

<AppBar border="border-b-2 border-surface-600">
	<svelte:fragment slot="lead">
		<div class="icon  ml-[20px] cursor-pointer select-none" use:popup={settingsClick}>
			<Options />
		</div>
		<div data-popup="settings-popup" class="card p-6 mt-5 -ml-5" >
			<div class="menu-item">
				<div class="icon">
					<UserIcon />
				</div>
				{#if $user_store}
					<a href={Routes.Account} class="text-lg underline hover:text-primary-300 transition-all">{$user_store.username}</a>
					{:else}
					<a href={Routes.Login}>Login</a>
				{/if}
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
			<hr class="my-4"/>
			{#if $user_store}
			<div class="menu-item">
				<div class="icon">
					<LogoutIcon />
				</div>
				<a href={Routes.Logout}>
					Logout
				</a>
			</div>
			{/if}
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