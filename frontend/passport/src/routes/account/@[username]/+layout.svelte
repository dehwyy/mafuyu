<script lang="ts">
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { Routes } from '$lib/const';
	import { page } from '$app/stores';

	const navigation = {
		"/": "Overview",
		"/statistics": "Statistics",
		"/edit":  "Settings"
	}

	let current_navigation: keyof typeof navigation = "/"
	$: {
		current_navigation = ($page.route.id?.split("/@[username]").at(-1) || "/") as keyof typeof navigation
	}
</script>

<div style="margin-right: calc(100% - 100vw - 100%)" class="w-full max-w-[1450px]">
	<div class="w-2/3 mx-auto pt-14 flex gap-x-16">
		<nav class="min-w-[230px] ">
			<section class="mb-7">
				<div class="w-[175px] h-[175px] object-cover overflow-hidden rounded-full  mb-4 border-2 mx-auto">
					<img alt="account_image" src="/images/r.jpg" />
				</div>
				<div class="text-center">
					<h4 class="h4">{$page.params.username}</h4>
					<h6 class="h6 opacity-50">Egor Hoshi</h6>
				</div>
			</section>
			<ListBox>
				{#each Object.entries(navigation) as entry}
					<a href={Routes.Account + `/@${$page.params.username}` + entry[0]} class="text-center text-lg mb-2 block">
						<ListBoxItem bind:group={current_navigation} name="theme" value={entry[0]}>{entry[1]}</ListBoxItem>
					</a>
				{/each}
			</ListBox>
		</nav>
		<main class="w-full pb-10">
			<slot />
		</main>
	</div>
</div>
