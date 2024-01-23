<script lang="ts">
  import ShareIconRaw from "$lib/assets/share.svg?raw"

  import OptionsPopup from "$lib/components/popups/options.svelte"

  const share_options = [{ icon: ShareIconRaw, text: "Copy link" }]
</script>

<OptionsPopup options={share_options} unique_target_name="share-options-popup">
  <button class="gap-x-[2px] share-button flex !outline-none items-center">
    <span class="icon-sm">
      {@html ShareIconRaw}
    </span>
    <span>Share</span>
  </button>
</OptionsPopup>
