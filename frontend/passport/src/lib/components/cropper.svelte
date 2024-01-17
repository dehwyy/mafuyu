<script lang="ts">
	import Cropper from 'cropperjs'
	import 'cropperjs/dist/cropper.css'

	interface CropperProps extends Cropper.Options<HTMLImageElement> {
		scaleX?: number
		scaleY?: number
		zoomTo?: number
		rotateTo?: number
	}

	export let cropper_props: CropperProps
	export let image_ref: HTMLImageElement | null = null
	export let cropper: Cropper | null

	$: cropper = image_ref
		? new Cropper(image_ref, {
				...cropper_props,
				ready: (event) => {
					const cropper = event.currentTarget.cropper

					cropper.zoomTo(cropper_props.zoomTo || 0)
					cropper.scaleY(cropper_props.scaleY || 1)
					cropper.scaleX(cropper_props.scaleX || 1)
					cropper.rotateTo(cropper_props.rotateTo || 0)

					cropper_props.ready && cropper_props.ready(event)
				}
			})
		: null

	export let src: string
	export let alt: string = 'cropper_image'
	export let style: string = ''

	// on `src` change, reset image
	$: {
		cropper?.reset().clear().replace(src)
	}

	// $: {
	// 	;[style]
	// 	cropper?.reset().clear().replace(src)
	// }

	export let classes: string = ''
</script>

<div {style} class={classes}>
	<img {src} {alt} bind:this={image_ref} style="opacity: 0; max-width: 100%" />
</div>
