<script lang="ts">
  import { Tree } from "$lib/components/tree"

  const n = [
    { h: "Node 2", children: [{ h: "Node 3", children: [{ h: "Node 4", children: [] }] }] },
    { h: "Node 22", children: [{ h: "Node 3", children: [{ h: "Node 4", children: [] }] }] },
    { h: "Node 22", children: [{ h: "Node 3", children: [{ h: "Node 4", children: [] }] }] },
  ]
</script>

<div class="w-full relative">
  <nav class="w-[300px] h-screen p-5 bg-surface-800 border-r-2 border-surface-500 fixed top-16 left-0">
    <div>
      <p>Latest</p>
      <ul>
        <li>Thus Spoke Zarathustra</li>
        <li>Chrestomathy of philosophy</li>
        <li>Nicomachean Ethics</li>
      </ul>
    </div>
  </nav>
  <section class="pl-[320px] w-full min-h-screen p-5">
    <div class="w-full max-w-[700px] flex flex-col gap-y-5">
      {#each [0, 0] as _}
        <Tree nodes={n}>Node 1</Tree>
      {/each}
    </div>
  </section>
</div>
