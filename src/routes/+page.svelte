<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { Maximize, Minimize, Minus, X } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { isMaximized } from '$lib/window.svelte';

	const appWindow = getCurrentWindow();

	appWindow.onResized(async () => {
		isMaximized.value = await appWindow.isMaximized();
	});

	let name = $state('');
	let greetMsg = $state('');

	async function greet(event: Event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		greetMsg = await invoke('greet', { name });
	}
</script>

<div class="flex flex-row-reverse gap-1 p-1" data-tauri-drag-region>
	<Button
		class="focus:bg-accent focus:text-accent-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
		variant="ghost"
		size="icon"
		onclick={appWindow.close}
	>
		<X />
	</Button>
	<Button
		class="focus:bg-accent focus:text-accent-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
		variant="ghost"
		size="icon"
		onclick={appWindow.toggleMaximize}
	>
		{#if isMaximized.value}
			<Minimize />
		{:else}
			<Maximize />
		{/if}
	</Button>
	<Button
		class="focus:bg-accent focus:text-accent-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
		variant="ghost"
		size="icon"
		onclick={appWindow.minimize}
	>
		<Minus />
	</Button>
</div>

<main class="container">
	<form class="row" onsubmit={greet}>
		<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>
</main>
