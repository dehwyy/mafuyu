<script>
  import { useUserInfo } from "$lib/query/user"
  import { authed_user_store } from "$lib/stores/user"
  import { spring } from "svelte/motion"
  import { DevFallbackImages } from "$lib/const"

  const [user, userStore] = useUserInfo({ oneofKind: "userId", userId: $authed_user_store?.id })
  $: userStore.set({ getBy: { oneofKind: "userId", userId: $authed_user_store?.id } })

  let minSize = 30
  let maxSize = 100

  let coords = spring(
    { x: 0, y: 0 },
    {
      stiffness: 0.1,
      damping: 0.25,
    },
  )

  let size = spring(minSize)

  $: size.set(minSize)
  $: maxSize = minSize + 50 >= maxSize ? minSize + 50 : maxSize
</script>

<div style="position: absolute; right: 4em; top: 6em" class="mt-20">
  <label>
    <h3>stiffness ({coords.stiffness})</h3>
    <input bind:value={coords.stiffness} type="range" min="0" max="1" step="0.01" />
  </label>

  <label>
    <h3>damping ({coords.damping})</h3>
    <input bind:value={coords.damping} type="range" min="0" max="1" step="0.01" />
  </label>

  <label>
    <h3>minSize ({minSize})</h3>
    <input bind:value={minSize} type="range" min="50" max="300" step="1" />
  </label>

  <label>
    <h3>maxSize ({maxSize})</h3>
    <input bind:value={maxSize} type="range" min={minSize + 50} max="600" step="1" />
  </label>
</div>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<svg on:mousemove={e => coords.set({ x: e.clientX, y: e.clientY })} on:mousedown={() => size.set(maxSize)} on:mouseup={() => size.set(minSize)}>
  <defs>
    <circle id="circle" cx={$coords.x} cy={$coords.y} r={$size / 2} />
    <clipPath id="circle-clip">
      <use xlink:href="#circle" />
    </clipPath>
  </defs>
  <g clip-path="url(#circle-clip)">
    <image
      xlink:href={$user.data?.picture || DevFallbackImages.HorizontalOriented}
      style="border-radius: 100%;"
      width={$size}
      height={$size}
      x={$coords.x - $size / 2}
      y={$coords.y - $size / 2}
      preserveAspectRatio="xMidYMid slice" />
  </g>
</svg>

<style>
  svg {
    width: 100vw;
    height: 100vh;
    margin-top: -64px;
  }
</style>
