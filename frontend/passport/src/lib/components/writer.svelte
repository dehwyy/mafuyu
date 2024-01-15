<script lang="ts">
	// @ts-ignore
	import CameraIcon from 'svelte-icons/fa/FaCamera.svelte'
	let text = ""
	let is_focused = false


</script>

<svelte:window on:click={() => is_focused = false} />
<div aria-hidden="true" class="w-full writer relative overflow-hidden" on:click={(e) => {
	e.stopPropagation()
	!is_focused && (is_focused = true)
}}>
	<div aria-hidden="true" class={`${text && "hidden"} ${!is_focused && "cursor-text"} absolute top-6 left-6 opacity-60 select-none writer-placeholder`}>
		What's new?
	</div>
	<div
		class="writer-input p-6"
		bind:innerText={text}
		contenteditable />
		<div class={`${is_focused || text ? "max-h-[100px]" : "max-h-0"} transition-all ease-linear duration-300 `}>
			<hr class="mb-4 mx-5"/>
			<div class="w-full flex items-center pl-8 pr-6 pb-4">
				<div class="">
					<div class="icon">
						<CameraIcon />
					</div>
				</div>
				<button disabled={!text} class="text-sm ml-auto block btn variant-filled-surface select-none">Create</button>
			</div>
		</div>
</div>

<style>
	.writer-placeholder {
		pointer-events: none;
	}

	.writer-input:focus-visible {
			outline: none;
	}

	.icon {
			width: 18px;
			height: 18px;
			cursor: pointer;
	}
</style>
