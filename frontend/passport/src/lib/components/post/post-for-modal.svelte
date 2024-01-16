<script lang="ts">
	import Writer from '$lib/components/writer.svelte'
	import OptionsComponent from './options.svelte'
	import LikeComponent from './like.svelte'
	import ShareComponent from './share.svelte'
	import CommentComponent from './comment.svelte'

	let comments = [
		{
			likes: 10,
			text: "i cant tell if you're joking or not",
			username: 'Waypo1nt',
			user_image: '/images/r.jpg'
		},
		{
			likes: 254,
			text: "i cant tell if you're joking or not",
			username: 'Waypo1nt',
			user_image: '/images/r.jpg'
		},
		{
			likes: 254,
			text: "i cant tell if you're joking or not",
			username: 'Waypo1nt',
			user_image: '/images/r.jpg'
		},
		{
			likes: 254,
			text: "i cant tell if you're joking or not",
			username: 'Waypo1nt',
			user_image: '/images/r.jpg'
		}
	]

	let post_content =
		'The propaganda level in legacy media has become tediously high, but also remarkable for how almost all legacy media repeat the same lies verbatim'
	let time = '21:17 UTC'
	let datetime = '2024-01-15T19:48:09.000Z'
	let date = '21 June, 2024'
	let views = '12.9K'
</script>

<section class="card p-6 max-w-full w-[600px] flex flex-col gap-y-3">
	<header class="flex items-center gap-x-3 w-full">
		<div class="object-cover w-[50px] h-[50px] rounded-full overflow-hidden">
			<img src="/images/r.jpg" alt="user" />
		</div>
		<div>
			<p class="font-[600]">dehwyy</p>
			<p class="opacity-60 text-sm">@dehwyy</p>
		</div>
		<div class="ml-auto">
			<OptionsComponent />
		</div>
	</header>
	<article class="font-[600]">
		{post_content}
	</article>
	<article class="flex text-sm mt-1">
		<time {datetime} class="opacity-50">
			{`${time} · ${date} ·`}&nbsp;
		</time>
		<p>{views} <span class="opacity-50">Views</span></p>
	</article>
	<hr />
	<article class="flex justify-between">
		<LikeComponent />
		<ShareComponent />
	</article>
	<article class="border-surface-400 dark:border-surface-700 rounded-lg">
		<Writer button_text="Comment" placeholder="Thoughts on post?" />
	</article>
	<section class="flex flex-col -mx-6 border-t border-surface-500">
		{#each comments as comment}
			<CommentComponent
				likes={comment.likes}
				username={comment.username}
				user_image={comment.user_image}
				text={comment.text}
			/>
		{/each}
	</section>
</section>
