import { Channel, invoke } from '@tauri-apps/api/core';

export enum Stage {
	INITIAL = 'initial',
	PICKING_FILE = 'pickingFile',
	MAILBOX_CONNECTING = 'mailboxConnecting',
	MAILBOX_CONNECTED = 'mailboxConnected',
	PROGRESS = 'progress',
	ERROR = 'error',
	FINISHED = 'finished'
}

export type WormholeEvent =
	| {
			event: Stage.MAILBOX_CONNECTING;
	  }
	| {
			event: Stage.PICKING_FILE;
	  }
	| {
			event: Stage.MAILBOX_CONNECTED;
			data: {
				code: string;
			};
	  }
	| {
			event: Stage.PROGRESS;
			data: {
				sent: number;
				total: number;
			};
	  };

export type ActiveProcess = null | 'send' | 'receive';

class Wormhole {
	active: ActiveProcess = $state(null);
	stage = $state(Stage.INITIAL);
	code = $state<string>();
	progress = $state({ sent: 0, total: 0 });
	error = $state<unknown>();

	async send_file(event: Event) {
		event.preventDefault();
		this.active = 'send';

		if (this.stage !== Stage.INITIAL) return;
		this.stage = Stage.MAILBOX_CONNECTING;

		const channel = new Channel<WormholeEvent>();
		channel.onmessage = (message) => {
			switch (message.event) {
				case Stage.MAILBOX_CONNECTING:
				case Stage.PICKING_FILE:
					break;
				case Stage.MAILBOX_CONNECTED:
					this.code = message.data.code;
					break;
				case Stage.PROGRESS:
					this.progress = message.data;
					break;
				default:
					console.error('Unknown message type', message);
					break;
			}

			// This assumes every possible stage in "message.event" is considered in the "Stage" enum
			this.stage = message.event;
		};

		try {
			await invoke('send_file', { channel });
			this.stage = Stage.FINISHED;
		} catch (e) {
			console.error(e);
			this.error = e;
			this.stage = Stage.ERROR;
		}
	}

	async receive_file(event: Event, code: string) {
		event.preventDefault();
		this.active = 'receive';

		if (this.stage !== Stage.INITIAL) return;
		this.stage = Stage.MAILBOX_CONNECTING;

		const channel = new Channel<WormholeEvent>();
		channel.onmessage = (message) => {
			switch (message.event) {
				case Stage.MAILBOX_CONNECTING:
				case Stage.MAILBOX_CONNECTED:
					break;
				case Stage.PROGRESS:
					this.progress = message.data;
					break;
				default:
					console.error('Unknown message type', message);
					break;
			}

			// This assumes every possible stage in "message.event" is considered in the "Stage" enum
			this.stage = message.event;
		};

		try {
			await invoke('receive_file', { channel, code });
			this.stage = Stage.FINISHED;
		} catch (e) {
			console.error(e);
			this.error = e;
			this.stage = Stage.ERROR;
		}
	}

	reset() {
		this.stage = Stage.INITIAL;
		this.code = '';
		this.progress.sent = 0;
		this.progress.total = 0;
		this.error = undefined;
		this.active = null;
	}
}

export const wormhole = new Wormhole();
