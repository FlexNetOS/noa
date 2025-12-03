import { type FSWatcher, watch } from "node:fs";
import type { Core } from "../core/backlog.ts";
import type { BacklogConfig } from "../types/index.ts";

export interface ConfigWatcherCallbacks {
	onConfigChanged?: (config: BacklogConfig | null) => void | Promise<void>;
}

export function watchConfig(core: Core, callbacks: ConfigWatcherCallbacks): { stop: () => void } {
	const configPath = core.filesystem.configFilePath;
	let watcher: FSWatcher | null = null;

	const stop = () => {
		if (watcher) {
			try {
				watcher.close();
			} catch {
				// Ignore
			}
			watcher = null;
		}
	};

	try {
		watcher = watch(configPath, async (eventType) => {
			if (eventType !== "change" && eventType !== "rename") {
				return;
			}
			try {
				core.filesystem.invalidateConfigCache();
				const config = await core.filesystem.loadConfig();
				await callbacks.onConfigChanged?.(config);
			} catch {
				// Ignore read errors; subsequent events will retry
			}
		});
	} catch {
		// If watching fails (e.g., file missing), keep silent; caller can retry via onConfigChanged
	}

	return { stop };
}
