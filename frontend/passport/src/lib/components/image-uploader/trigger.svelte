<script lang="ts">
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton'
	import ImageUploaderComponent from './modal.svelte'

	export let images: string[]

	const modal_store = getModalStore()

	const image_uploader_modal_settings: ModalSettings = {
		title: 'ImageUploader',
		type: 'component',
		component: { ref: ImageUploaderComponent },
		response: (v: string[] | undefined) => {
			if (v && v.length + images.length <= 10) images = [...images, ...v]
		}
	}
</script>

<section
	aria-hidden="true"
	on:click={() => modal_store.set([image_uploader_modal_settings, ...$modal_store])}
>
	<slot />
</section>
