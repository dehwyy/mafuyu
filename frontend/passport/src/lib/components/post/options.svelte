<script lang="ts">
  import HorizontalDotsIconRaw from "$lib/assets/dots-horizontal.svg?raw"
  import BlockIconRaw from "$lib/assets/block.svg?raw"
  import HiddenIconRaw from "$lib/assets/hidden.svg?raw"
  import FlagIconRaw from "$lib/assets/flag.svg?raw"
  import WorldIconRaw from "$lib/assets/world.svg?raw"

  import OptionsPopup from "$lib/components/popups/options.svelte"

  const post_options = [
    { icon: BlockIconRaw, text: "Block @dehwyy" },
    { icon: HiddenIconRaw, text: "Hide the post" },
    { icon: FlagIconRaw, text: "Report" },
    { icon: WorldIconRaw, text: "Translate (beta)" },
  ]
</script>

<OptionsPopup unique_target_name="post-options-popup" options={post_options} placement="bottom">
  <button class="hover:bg-gray-500/40 !outline-none rounded-full p-[3px] icon-md">
    {@html HorizontalDotsIconRaw}
  </button>
</OptionsPopup>
