<script lang="ts">
	import ThumbsUpIcon from '$lib/assets/thumbs_up.svg?raw';
	export let likes = 13;

	let is_liked = false;
	const ToggleLikePost = () => {
		is_liked ? (likes -= 1) : (likes += 1);
		is_liked = !is_liked;
	};
</script>

<button on:click={ToggleLikePost} class="like-button flex gap-x-1 !outline-none items-center">
	<span class={`${is_liked && '!fill-primary-400 dark:!fill-primary-500'} icon`}>
		{@html ThumbsUpIcon}
	</span>
	<span>{likes}</span>
</button>

<style>
	.like-button:hover .icon {
		@apply bg-gray-500/40;
	}
	.icon {
		@apply mt-[-2px] w-[24px] h-[24px] block p-[4px];
		@apply rounded-full transition-all dark:stroke-white stroke-black fill-none stroke-2;
	}
</style>
