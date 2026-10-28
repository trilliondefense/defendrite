
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  onMount(() => {
    const userName = localStorage.getItem("userName");

    if (userName?.trim()) {
      goto("/app/newchat", { replaceState: true });
    } else {
      goto("/onboard", { replaceState: true });
    }
  });
</script>

<div class="loading-screen">
  <div class="loading-content">
    <img src="/favicon.png" alt="" class="message-avatar" />

    <p>Loading your workspace...</p>
  </div>
</div>

<style>
  .loading-screen {
    min-height: 100dvh;
    display: grid;
    place-items: center;
    background: #fbfaf7;
    color: #292824;
  }

  .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  

.message-avatar {
  width: 27px;
  height: 27px;
  object-fit: contain;
}

  p {
    margin: 0;
    color: #89867e;
    font-size: 12px;
  }
</style>
