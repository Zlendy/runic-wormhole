<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Maximize, Minimize, Minus, X } from '@lucide/svelte';

  const appWindow = getCurrentWindow();

  appWindow.onResized(async () => {
    isMaximized = await appWindow.isMaximized();
  });

  let name = $state("");
  let greetMsg = $state("");
  let isMaximized = $state(false);

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="p-4 flex gap-4 flex-row-reverse" data-tauri-drag-region>
  <button onclick={appWindow.close}>
    <X />
  </button>
  <button onclick={appWindow.toggleMaximize}>
    {#if isMaximized}
      <Minimize />
    {:else}
      <Maximize />
    {/if}
  </button>
  <button onclick={appWindow.minimize}>
    <Minus />
  </button>
</div>

<main class="container">
  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</main>
