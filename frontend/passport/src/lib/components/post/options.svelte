<script lang="ts">
	import HorizontalDotsIcon from 'svelte-icons/fa/FaEllipsisH.svelte'
	import BlockIcon from 'svelte-icons/md/MdBlock.svelte'
	import HiddenIcon from 'svelte-icons/io/IoIosEyeOff.svelte'
	import FlagIcon from 'svelte-icons/fa/FaRegFlag.svelte'
	import WorldIcon from 'svelte-icons/gi/GiWorld.svelte'

	import OptionsPopup from '$lib/components/popups/options.svelte'

	const post_options = [
		{ icon: BlockIcon, text: 'Block @dehwyy' },
		{ icon: HiddenIcon, text: 'Hide the post' },
		{ icon: FlagIcon, text: 'Report' },
		{ icon: WorldIcon, text: 'Translate (beta)' }
	]
</script>

<OptionsPopup unique_target_name="post-options-popup" options={post_options} placement="bottom">
	<button class="hover:bg-gray-500/40 !outline-none rounded-full p-[3px] icon-md">
		<HorizontalDotsIcon />
	</button>
</OptionsPopup>
