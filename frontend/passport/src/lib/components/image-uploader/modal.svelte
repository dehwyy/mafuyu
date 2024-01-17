<script lang="ts">
	import { FileButton, getModalStore, SlideToggle } from '@skeletonlabs/skeleton'
	import Cropper from '$lib/components/cropper.svelte'
	import CropperJS from 'cropperjs'
	import { FileReaderTool } from '$lib/utils/file_reader'

	const modal_store = getModalStore()
	const MAX_IMAGES = 10

	export let parent: any

	let selected_file_images: FileList

	let unprocessed_images: string[] = []
	let processed_images: string[] = []
	let current_image: string | null = null

	let cropper: CropperJS | null = null
	let is_fullscreen_image = false
	let is_ready = false

	const OnFileSelect = async (e: Event) => {
		const input = e.target as HTMLInputElement
		if (!input.files) {
			console.log("no files provided")
			return
		}

		const files_count = input.files.length
		const loaded_images: string[] = new Array(files_count) // base64[]
		for (let i = 0; i < files_count; i++) {
			const image_to_load = input.files.item(i)! // should exist

			// image_to_load.size > 10 * 8 // TODO: add size
			// ["image/jpeg", "image/png"].includes(image_to_load.type) // TODO: add .ext validation

			loaded_images[files_count - i - 1] = await FileReaderTool.AsBase64(image_to_load)
		}

		//
		if (loaded_images.length + processed_images.length <= MAX_IMAGES) {
			unprocessed_images = [...unprocessed_images, ...loaded_images]
		}
	}
	const SaveImage = () => {
		const modal = $modal_store[0]

		if (!modal || !modal.response || !cropper) return

		const new_processed_images = [...processed_images, cropper.getCroppedCanvas().toDataURL()]

		if (unprocessed_images.length > 0 && cropper) {
			processed_images = new_processed_images
			return
		}

		modal.response(new_processed_images)
		modal_store.close()
	}


	$: {
		[unprocessed_images.length, processed_images.length]
		if (unprocessed_images.length > 0) {
			current_image = unprocessed_images.pop()!
		}
	}

	// set `is_ready` on:imageSet
	$: {
		is_ready = !!current_image
	}

	// if image display settings is changed, render cropper
	$: {
		;[is_fullscreen_image] // deps

		if (cropper && current_image) {
			is_ready = false

			const img = current_image
			const cloned_cropper = cropper
			setTimeout(() => {
				is_ready = true
				cloned_cropper.reset().clear().replace(img)
			}, 150)
		}
	}
</script>

<section class="card p-6 flex flex-col gap-y-5">
	<h4 class="h4">Choose photo</h4>
	{#if current_image}
		<section
			class={`${
				is_ready ? 'opacity-100 visible max-h-[800px]' : 'opacity-0 invisible max-h-[450px]'
			} transition-all overflow-hidden flex flex-col gap-y-5 items-center pb-5 border-b-2 border-b-surface-500`}
		>
			<ul>
				<li class="flex gap-x-3 items-center">
					<SlideToggle name="fullscreen_image" bind:checked={is_fullscreen_image} />
					<p class="font-[500]">Fullscreen Mode</p>
				</li>
			</ul>
			<Cropper
				bind:cropper
				src={current_image}
				style={`width: ${!is_fullscreen_image ? 'min(600px, 90vw)' : 'min(600px, 90vw)'} ;height: ${
					!is_fullscreen_image ? 'calc(100vh - 350px)' : ''
				}`}
				cropper_props={{
					initialAspectRatio: 1,
					aspectRatio: 1,
					viewMode: 2,
					dragMode: 'crop'
				}}
			/>
			<button class="btn variant-filled-surface w-full lg:w-1/2" on:click={SaveImage}
				>Save image</button
			>
		</section>
	{/if}
	<FileButton
		multiple
		button={`btn variant-soft-primary w-full mx-auto block ${current_image ? 'lg:w-1/2' : ''}`}
		name="image-uploader"
		bind:files={selected_file_images}
		on:change={OnFileSelect}
	/>
</section>
