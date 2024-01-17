<script lang="ts">
	import CameraIcon from 'svelte-icons/fa/FaCamera.svelte'
	import ImageUploader from './image-uploader/trigger.svelte'

	export let button_text: string
	export let placeholder: string
	export let padding: string = '1.5rem'

	let text = ''
	let image = ''
	let is_focused = false
</script>

<svelte:window on:click={() => (is_focused = false)} />
<div
	aria-hidden="true"
	class="w-full writer relative overflow-hidden focus-visible:!outline-none"
	on:click={(e) => {
		e.stopPropagation()
		!is_focused && (is_focused = true)
	}}
>
	<div
		aria-hidden="true"
		style={`top: ${padding};left: ${padding}`}
		class={`${text && 'hidden'} ${
			!is_focused && 'cursor-text'
		} absolute top-6 left-6 opacity-60 select-none pointer-events-none`}
	>
		{placeholder}
	</div>
	<div
		style={`padding:${padding}`}
		class={`writer-input !outline-none`}
		bind:innerText={text}
		contenteditable
	/>
	{#if image}
		<div class="max-w-full max-h-[500px] object-cover overflow-hidden p-5 grid place-items-center">
			<img src={image} alt="writer" class="rounded-xl" />
		</div>
	{/if}
	<div
		class={`${
			is_focused || text ? 'max-h-[100px]' : 'max-h-0'
		} transition-all ease-linear duration-300 `}
	>
		<hr class="mb-4 mx-5" />
		<div class="w-full flex items-center pl-8 pr-6 pb-4">
			<div class="">
				<ImageUploader bind:image>
					<button class="icon-sm">
						<CameraIcon />
					</button>
				</ImageUploader>
			</div>
			<button
				disabled={!(text || image)}
				class="text-sm ml-auto block btn variant-filled-surface select-none">{button_text}</button
			>
		</div>
	</div>
</div>
