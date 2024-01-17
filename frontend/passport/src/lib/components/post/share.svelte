<script lang="ts">
	// @ts-ignore
	import ShareIcon from 'svelte-icons/md/MdShare.svelte'

	import OptionsPopup from '$lib/components/popups/options.svelte'

	const share_options = [{ icon: ShareIcon, text: 'Copy link' }]
</script>

<OptionsPopup options={share_options} unique_target_name="share-options-popup">
	<button class="gap-x-[2px] share-button flex !outline-none items-center">
		<span class="icon-sm">
			<ShareIcon />
		</span>
		<span>Share</span>
	</button>
</OptionsPopup>
