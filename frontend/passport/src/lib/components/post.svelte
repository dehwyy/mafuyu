<script lang="ts">
  import CommentIcon from "svelte-icons/fa/FaRegComment.svelte"
  import StatsIcon from "svelte-icons/io/IoIosStats.svelte"

  import LikeComponent from "$lib/components/post/like.svelte"
  import ShareComponent from "$lib/components/post/share.svelte"
  import OptionsComponent from "$lib/components/post/options.svelte"

  export let post_id = "123"
  export let user_image = "/images/r.jpg"
  export let username = "dehwyy"
  export let post_date = "21 June"
  export let post_content =
    "The propaganda level in legacy media has become tediously high, but also remarkable for how almost all legacy media repeat the same lies verbatim"

  export let post_statistics = {
    likes: 14,
    comments: 2,
    views: 2185,
  }

  const post_href = `@${username}/${post_id}`
</script>

<div class="w-full card p-4 flex gap-x-3">
  <aside>
    <div class="object-cover w-[48px] h-[48px] rounded-full overflow-hidden">
      <img src={user_image} alt={username} />
    </div>
  </aside>
  <article class="flex-auto">
    <header class="h-[35px] flex gap-x-2 w-full">
      <p class="font-[600]">{username}</p>
      <p>·</p>
      <p class="opacity-70">{post_date}</p>
    </header>
    <a href={post_href}>
      <div class="font-[500] cursor-pointer">
        {post_content}
      </div>
    </a>
    <footer class="text-sm mt-4">
      <ul class="analytics-bar grid grid-cols-4 pr-2">
        <li>
          <LikeComponent />
        </li>
        <li>
          <a href={post_href}>
            <button>
              <span class="icon">
                <CommentIcon />
              </span>
              <span>{post_statistics.comments}</span>
            </button>
          </a>
        </li>
        <li>
          <a href={post_href}>
            <button>
              <span class="icon">
                <StatsIcon />
              </span>
              <span>{post_statistics.views}</span>
            </button>
          </a>
        </li>
        <li class="ml-auto">
          <ShareComponent />
        </li>
      </ul>
    </footer>
  </article>
  <aside class="w-[70px] mt-0.5">
    <OptionsComponent />
  </aside>
</div>

<style>
  .icon {
    margin-top: -2px;
    width: 24px;
    height: 24px;
    display: block;
    padding: 4px;
    @apply rounded-full transition-all dark:stroke-white stroke-black fill-none stroke-2;
  }

  .analytics-bar li:hover .icon {
    @apply bg-gray-500/40;
  }

  .analytics-bar button {
    display: flex;
    gap: 0.5rem;
  }
</style>
