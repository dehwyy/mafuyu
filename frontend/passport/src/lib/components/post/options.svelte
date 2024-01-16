<script lang="ts">
	// @ts-ignore
	import HorizontalDotsIcon from 'svelte-icons/fa/FaEllipsisH.svelte'
	// @ts-ignore
	import BlockIcon from 'svelte-icons/md/MdBlock.svelte'
	// @ts-ignore
	import HiddenIcon from 'svelte-icons/io/IoIosEyeOff.svelte'
	// @ts-ignore
	import FlagIcon from 'svelte-icons/fa/FaRegFlag.svelte'
	import  { type PopupSettings, popup} from '@skeletonlabs/skeleton';

	const moreOptionsSettings: PopupSettings = {
		target: "more-options-popup",
		event: "click",
		placement: "bottom"
	}
</script>

<button class="icon hover:bg-gray-500/40 !outline-none" use:popup={moreOptionsSettings}>
	<HorizontalDotsIcon />
</button>
<div data-popup="more-options-popup" class="card border border-surface-600">
	<ul class="list options-popup py-2 px-1 select-none">
		<li>
			<div class="options-icon">
				<BlockIcon />
			</div>
			<p>Block @dehwyy</p>
		</li>
		<li>
			<div class="options-icon">
				<HiddenIcon />
			</div>
			<p>Hide the post</p>
		</li>
		<li>
			<div class="options-icon">
				<FlagIcon />
			</div>
			<p>Report</p>
		</li>
	</ul>
</div>

<style>
    .icon {
        @apply mt-[-2px] w-[24px] h-[24px] block p-[4px] rounded-full transition-all;
    }

    .options-popup li {
        @apply px-5 py-2 font-[600] cursor-pointer dark:hover:bg-surface-700 hover:bg-surface-200 transition-all;
    }

		.options-icon {
				@apply w-[20px] h-[20px];
		}
</style>