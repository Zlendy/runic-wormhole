<script lang="ts">
	import { Channel, invoke } from '@tauri-apps/api/core';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import Button from '$lib/components/ui/button/button.svelte';
	import Progress from '$lib/components/ui/progress/progress.svelte';

	type WormholeEvent =
		| {
				event: 'mailboxConnected';
				data: {
					code: string;
				};
		  }
		| {
				event: 'progress';
				data: {
					sent: number;
					total: number;
				};
		  }
		| {
				event: 'finished';
		  };

	let path = $state('');
	let code = $state<string>();
	let progress = $state({ sent: 0, total: 0 });
	let error = $state<unknown>();

	async function send_file(event: Event) {
		event.preventDefault();

		const channel = new Channel<WormholeEvent>();
		channel.onmessage = (message) => {
			switch (message.event) {
				case 'mailboxConnected':
					code = message.data.code;
					break;
				case 'progress':
					progress = message.data;
					break;
				case 'finished':
					break;
				default:
					console.error('Unknown message type', message);
					break;
			}
		};

		try {
			const response = await invoke('send_file', { channel });
			console.log('response', response);
		} catch (e) {
			console.error(e);
			error = e;
		}
	}
</script>

<div class="grid w-full max-w-sm items-center gap-1.5">
	<Button onclick={send_file}>Send</Button>

	<Label for="code">Code</Label>
	<Input id="code" bind:value={code} readonly />

	{progress.sent}/{progress.total}
	<Progress value={progress.sent} max={progress.total} />

	Error: {error}
</div>
