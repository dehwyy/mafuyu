<script lang="ts">
	import CommentIcon from 'svelte-icons/fa/FaRegComment.svelte'
	import StatsIcon from 'svelte-icons/io/IoIosStats.svelte'

	import PostForModalComponent from '$lib/components/post/post-for-modal.svelte'
	import LikeComponent from '$lib/components/post/like.svelte'
	import ShareComponent from '$lib/components/post/share.svelte'
	import OptionsComponent from '$lib/components/post/options.svelte'
	import { getModalStore, type ModalSettings, type ModalComponent } from '@skeletonlabs/skeleton'

	const modal_store = getModalStore()

	const modal_component: ModalComponent = { ref: PostForModalComponent }

	const postModalSettings: ModalSettings = {
		type: 'component',
		component: modal_component,
		title: 'post'
	}

	export let user_image = '/images/r.jpg'
	export let username = 'dehwyy'
	export let post_date = '21 June'
	export let post_content =
		'The propaganda level in legacy media has become tediously high, but also remarkable for how almost all legacy media repeat the same lies verbatim'

	export let post_statistics = {
		likes: 14,
		comments: 2,
		views: 2185
	}

	const OpenPostAsModal = () => modal_store.trigger(postModalSettings)
</script>

<div class="w-full card p-4 flex gap-x-3">
	<aside>
		<div class="object-cover w-[48px] h-[48px] rounded-full overflow-hidden">
			<img src={user_image} alt={username} />
		</div>
	</aside>
	<article class="flex-auto">
		<header class="h-[35px] flex gap-x-2 w-full">
			<p class="font-[600]">{username}</p>
			<p>·</p>
			<p class="opacity-70">{post_date}</p>
		</header>
		<div on:click={OpenPostAsModal} class="font-[500] cursor-pointer">
			{post_content}
		</div>
		<footer class="text-sm mt-4">
			<ul class="analytics-bar grid grid-cols-4 pr-2">
				<li>
					<LikeComponent />
				</li>
				<li>
					<button on:click={OpenPostAsModal}>
						<span class="icon">
							<CommentIcon />
						</span>
						<span>{post_statistics.comments}</span>
					</button>
				</li>
				<li>
					<button on:click={OpenPostAsModal}>
						<span class="icon">
							<StatsIcon />
						</span>
						<span>{post_statistics.views}</span>
					</button>
				</li>
				<li class="ml-auto">
					<ShareComponent />
				</li>
			</ul>
		</footer>
	</article>
	<aside class="w-[70px] mt-0.5">
		<OptionsComponent />
	</aside>
</div>

<style>
	.icon {
		margin-top: -2px;
		width: 24px;
		height: 24px;
		display: block;
		padding: 4px;
		@apply rounded-full transition-all dark:stroke-white stroke-black fill-none stroke-2;
	}

	.analytics-bar li:hover .icon {
		@apply bg-gray-500/40;
	}

	.analytics-bar button {
		display: flex;
		gap: 0.5rem;
	}
</style>
