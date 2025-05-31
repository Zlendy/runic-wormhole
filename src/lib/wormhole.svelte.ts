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

let stage = $state(Stage.INITIAL);
let code = $state<string>();
let progress = $state({ sent: 0, total: 0 });
let error = $state<unknown>();

export const wormhole = {
	get stage() {
		return stage;
	},

	get code() {
		return code;
	},

	get progress() {
		return progress;
	},

	get error() {
		return error;
	},

	async send_file(event: Event) {
		event.preventDefault();

		if (stage !== Stage.INITIAL) return;
		stage = Stage.MAILBOX_CONNECTING;

		const channel = new Channel<WormholeEvent>();
		channel.onmessage = (message) => {
			switch (message.event) {
				case Stage.MAILBOX_CONNECTING:
				case Stage.PICKING_FILE:
					break;
				case Stage.MAILBOX_CONNECTED:
					code = message.data.code;
					break;
				case Stage.PROGRESS:
					progress = message.data;
					break;
				default:
					console.error('Unknown message type', message);
					break;
			}

			// This assumes every possible stage in "message.event" is considered in the "Stage" enum
			stage = message.event;
		};

		try {
			await invoke('send_file', { channel });
			stage = Stage.FINISHED;
		} catch (e) {
			console.error(e);
			error = e;
			stage = Stage.ERROR;
		}
	},

	async receive_file(event: Event, code: string) {
		event.preventDefault();

		if (stage !== Stage.INITIAL) return;
		stage = Stage.MAILBOX_CONNECTING;

		const channel = new Channel<WormholeEvent>();
		channel.onmessage = (message) => {
			switch (message.event) {
				case Stage.MAILBOX_CONNECTING:
				case Stage.MAILBOX_CONNECTED:
					break;
				case Stage.PROGRESS:
					progress = message.data;
					break;
				default:
					console.error('Unknown message type', message);
					break;
			}

			// This assumes every possible stage in "message.event" is considered in the "Stage" enum
			stage = message.event;
		};

		try {
			await invoke('receive_file', { channel, code });
			stage = Stage.FINISHED;
		} catch (e) {
			console.error(e);
			error = e;
			stage = Stage.ERROR;
		}
	},

	reset() {
		stage = Stage.INITIAL;
		code = '';
		progress.sent = 0;
		progress.total = 0;
		error = undefined;
	}
};
