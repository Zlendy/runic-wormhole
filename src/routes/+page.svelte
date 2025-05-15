<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import Button from '$lib/components/ui/button/button.svelte';

	let path = $state('');

	async function send_file(event: Event) {
		event.preventDefault();

		const response = await invoke('send_file', { path });
		console.log('response', response);
	}
</script>

<div class="grid w-full max-w-sm items-center gap-1.5">
	<!-- <Label for="files">Files</Label>
	<Input id="files" type="file" bind:files /> -->
	<Label for="path">Path</Label>
	<Input id="path" bind:value={path} />
	<Button onclick={send_file}>Send</Button>
</div>
