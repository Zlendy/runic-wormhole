<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import Button from '$lib/components/ui/button/button.svelte';
	import Progress from '$lib/components/ui/progress/progress.svelte';
	import CircleAlert from '@lucide/svelte/icons/circle-alert';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { emit } from '@tauri-apps/api/event';

	import { Stage, wormhole } from '$lib/wormhole.svelte';
	import { codeLength } from '$lib/localstorage.svelte';
</script>

{#snippet cancel()}
	<Button variant="destructive" onclick={() => emit('cancel')}>Cancel</Button>
{/snippet}

<div class="grid w-full max-w-sm items-center gap-1.5">
	{#if [Stage.INITIAL, Stage.PICKING_FILE].includes(wormhole.stage)}
		<Button onclick={(e) => wormhole.send_file(e, $codeLength)} disabled={wormhole.stage !== Stage.INITIAL}>
			Send
		</Button>
	{:else if wormhole.stage === Stage.MAILBOX_CONNECTING}
		Connecting...
	{:else if wormhole.stage === Stage.MAILBOX_CONNECTED}
		<Label for="code">Code</Label>
		<Textarea id="code" value={wormhole.code} readonly />
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
		<Button onclick={() => wormhole.reset()}>Try again</Button>
	{:else if wormhole.stage === Stage.FINISHED}
		File sent successfully
		<Button onclick={() => wormhole.reset()}>Send more</Button>
	{:else}
		INVALID STAGE "{wormhole.stage}"
		<Button onclick={() => wormhole.reset()}>Try again</Button>
	{/if}
</div>
