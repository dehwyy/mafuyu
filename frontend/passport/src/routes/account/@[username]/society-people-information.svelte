<script lang="ts">
	export let label: string
	export let value: string | number
	export let see_all_href: string

	// export let value_array: unknown[]
</script>

<slot />
<p class="flex w-full items-center">
	{label}: &nbsp; <span class="text-primary-500 underline">{value}</span>
	<span class="ml-6 flex">
		{#each Array(3) as _, i}
			<span class="block ml-[-0.25rem] border-2 border-surface-100 dark:border-surface-800 h-[30px] w-[30px] rounded-full overflow-hidden">
				<img src="/images/r.jpg" alt="friend_image" />
			</span>
		{/each}
	</span>
	<a href={see_all_href} class="block ml-auto hover:underline transition-all">see all</a>
</p>
