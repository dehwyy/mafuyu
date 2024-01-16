<script lang="ts">
	// @ts-ignore
	import ShareIcon from 'svelte-icons/md/MdShare.svelte';
</script>

<button class="gap-x-[2px] share-button flex !outline-none items-center">
	<span class="icon">
		<ShareIcon />
	</span>
	<span>Share</span>
</button>

<style>
	.share-button:hover .icon {
		@apply bg-gray-500/40;
	}
	.icon {
		@apply mt-[-2px] w-[24px] h-[24px] block p-[4px] rounded-full;
	}
</style>
