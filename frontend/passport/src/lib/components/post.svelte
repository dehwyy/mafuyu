<script lang="ts">
	// @ts-ignore
	import ThumbsUpIcon from 'svelte-icons/fa/FaRegThumbsUp.svelte'
	// @ts-ignore
	import CommentIcon from 'svelte-icons/fa/FaRegComment.svelte'
	// @ts-ignore
	import StatsIcon from 'svelte-icons/io/IoIosStats.svelte'
	// @ts-ignore
	import BookmarkIcon from 'svelte-icons/io/IoMdBookmark.svelte'

	export let user_image = "/images/r.jpg"
	export let username = "dehwyy"
	export let post_date = "21 June"
	export let post_content = "The propaganda level in legacy media has become tediously high, but also remarkable for how almost all legacy media repeat the same lies verbatim"

	export let post_statistics = {
		likes: 14,
		comments: 2,
		views: 2185,
	}

</script>

<div class="w-full card p-4 flex gap-x-3">
	<aside class="">
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
		<div class="font-[500]">
			{post_content}
		</div>
		<footer class="text-sm mt-4">
			<ul class="analytics-bar flex justify-between pr-2">
				<li>
					<button>
						<span class="icon">
							<ThumbsUpIcon />
						</span>
						<span>{post_statistics.likes}</span>
					</button>
				</li>
				<li>
					<button>
						<span class="icon">
							<CommentIcon />
						</span>
						<span>{post_statistics.comments}</span>
					</button>
				</li>
				<li>
					<button>
						<span class="icon">
							<StatsIcon />
						</span>
						<span>{post_statistics.views}</span>
					</button>
				</li>
				<li>
					<button class="icon">
						<BookmarkIcon />
					</button>
				</li>
			</ul>
		</footer>
	</article>
	<aside class="w-[70px]">

	</aside>
</div>


<style>
	.icon {
			margin-top: 1px;
			width: 16px;
			height: 16px;
			display: block;
	}

	.analytics-bar li button {
			display: flex;
			gap: 0.5rem;
	}

</style>