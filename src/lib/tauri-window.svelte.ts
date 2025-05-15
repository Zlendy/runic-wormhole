import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();
let _isMaximized = $state({ value: false });

appWindow.onResized(async () => {
	_isMaximized.value = await appWindow.isMaximized();
});

export default abstract class TauriWindow {
	public static get isMaximized() {
		return _isMaximized.value;
	}

	public static close = appWindow.close;
	public static toggleMaximize = appWindow.toggleMaximize;
	public static minimize = appWindow.minimize;
}
