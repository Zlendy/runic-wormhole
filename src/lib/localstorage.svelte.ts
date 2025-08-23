import { persisted } from 'svelte-persisted-store';

export const codeLength = persisted('codeLength', 4);
