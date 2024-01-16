<script lang="ts">
	import LikeComponent from './like.svelte'
	export let likes = 0
	export let text: string
	export let comment_id: string
	export let username: string
	export let pseudonym = username
	export let user_image: string
	export let comment_time = new Date(Date.now()).getHours()

	// either BOTH ids id provided OR nothing
	export let as_response_to_user = '' // it's  userId
	export let as_response_to_comment = '' // it's a commentId
	//
</script>

<section class="w-full border-b p-4 pb-3 border-surface-500">
	<header class="flex gap-x-3">
		<div class="object-cover w-[40px] h-[40px] rounded-full overflow-hidden">
			<img src={user_image} alt={username} />
		</div>
		<div class="flex gap-x-2 text-sm pl-[2px]">
			<p class="font-[900]">{pseudonym}</p>
			<p class="opacity-50">@{username} · {comment_time} hours ago</p>
			<p class="opacity-50"></p>
		</div>
	</header>
	<article class="font-[600] pl-14 mt-[-10px] text-sm">
		{text}
	</article>
	<footer class="pl-14 text-sm mt-3">
		<LikeComponent />
	</footer>
</section>
