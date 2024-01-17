<script lang="ts">
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton'
	import ImageUploaderComponent from './modal.svelte'

	const modal_store = getModalStore()

	const image_uploader_modal_settings: ModalSettings = {
		title: 'ImageUploader',
		type: 'component',
		component: { ref: ImageUploaderComponent }
	}
</script>

<section aria-hidden="true" on:click={() => modal_store.trigger(image_uploader_modal_settings)}>
	<slot />
</section>
