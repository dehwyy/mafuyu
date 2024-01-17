<script lang="ts">
	import { FileDropzone, getModalStore, SlideToggle } from '@skeletonlabs/skeleton'
	import Cropper from '$lib/components/cropper.svelte'
	import CropperJS from 'cropperjs'

	const modal_store = getModalStore()
	export let parent: any

	let selected_file_images: FileList
	let image: string | null = null
	let cropper: CropperJS | null = null

	const readFileAsBase64 = async (file: File) => {
		return new Promise<string>((resolve) => {
			const reader = new FileReader()
			reader.readAsDataURL(file)
			reader.onload = async () => {
				const image = reader.result
				return resolve(String(image))
			}
		})
	}

	const OnFileSelect = (e: Event) => {
		const input = e.target as HTMLInputElement
		const image_to_load = input.files?.item(0)

		if (!image_to_load) return

		// image_to_load.size > 10 * 8 // TODO: add size
		// ["image/jpeg", "image/png"].includes(image_to_load.type) // TODO: add .ext validation
		readFileAsBase64(image_to_load).then((base64file) => (image = base64file))
	}
	const SaveImage = () => {
		const modal = $modal_store.find((v) => v.title === 'ImageUploader')
		if (!modal || !modal.response) return
		modal.response(cropper?.getCroppedCanvas().toDataURL())
		modal_store.close()
	}

	let is_fullscreen_image = false
	let is_ready = false

	$: {
		is_ready = !!image
	}

	$: {
		;[is_fullscreen_image]
		if (cropper && image) {
			is_ready = false

			const img = image
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
	{#if image}
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
				src={image}
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
	<FileDropzone name="image-uploader" bind:files={selected_file_images} on:change={OnFileSelect} />
</section>