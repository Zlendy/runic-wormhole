<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import Button from '$lib/components/ui/button/button.svelte';
	import Progress from '$lib/components/ui/progress/progress.svelte';
	import CircleAlert from '@lucide/svelte/icons/circle-alert';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { emit } from '@tauri-apps/api/event';

	import { Stage, wormhole } from '$lib/wormhole.svelte';

	let code = $state('');
</script>

{#snippet cancel()}
	<Button variant="destructive" onclick={() => emit('cancel')}>Cancel</Button>
{/snippet}

<div class="grid w-full max-w-sm items-center gap-1.5">
	{#if wormhole.stage === Stage.INITIAL}
		<Label for="code">Code</Label>
		<Input id="code" bind:value={code} />
		<Button onclick={(e) => wormhole.receive_file(e, code)} disabled={code.trim() === ''}
			>Receive</Button
		>
	{:else if wormhole.stage === Stage.MAILBOX_CONNECTING}
		Connecting...
	{:else if wormhole.stage === Stage.MAILBOX_CONNECTED}
		Select where to save the incoming file
		{@render cancel()}
	{:else if wormhole.stage === Stage.PROGRESS}
		{wormhole.progress.sent}/{wormhole.progress.total}
		<Progress value={wormhole.progress.sent} max={wormhole.progress.total} />
		{@render cancel()}
	{:else if wormhole.stage === Stage.ERROR}
		<Alert.Root variant="destructive">
			<CircleAlert class="size-4" />
			<Alert.Title>Error</Alert.Title>
			<Alert.Description>{wormhole.error}</Alert.Description>
		</Alert.Root>
		<Button onclick={wormhole.reset}>Try again</Button>
	{:else if wormhole.stage === Stage.FINISHED}
		File received successfully
		<Button onclick={wormhole.reset}>Receive more</Button>
	{:else}
		INVALID STAGE "{wormhole.stage}"
		<Button onclick={wormhole.reset}>Try again</Button>
	{/if}
</div>
