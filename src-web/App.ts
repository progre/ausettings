declare const window: Window & {
  external: { invoke(arg: string): void };
};

function invoke<T>(type: string, payload: any): Promise<T> {
  return new Promise((resolve, reject) => {
    const callback = `_${Math.floor(
      ((Math.random() + 1) / 2) * Number.MAX_SAFE_INTEGER,
    )}`;
    (<any>window)[callback] = (err: any, value: T) => {
      delete (<any>window)[callback];
      if (err != null) {
        reject(err);
        return;
      }
      resolve(value);
    };
    window.external.invoke(
      JSON.stringify({
        type,
        callback,
        payload,
      }),
    );
  });
}

interface GameSettingsListItem {
  name: string;
  gameSettings: Object | null;
}

export default class App {
  gameSettingsList() {
    return invoke<readonly GameSettingsListItem[]>('game_settings_list', {});
  }

  setGameSettingsName(index: number, name: string) {
    return invoke<void>('set_game_settings_name', { index, name });
  }

  saveMemoryToFile(index: number) {
    return invoke<void>('save_memory_to_file', { index });
  }

  loadMemoryFromFile(index: number) {
    return invoke<void>('load_memory_from_file', { index });
  }
}
