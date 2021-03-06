export interface ProcessStatus {
  auCaptureOffsets: boolean;
  auProcess: boolean;
}

declare const window: Window & {
  external: { invoke(arg: string): void };
  onChangeProcessStatus: ((status: ProcessStatus) => void) | null;
};

function generateUniqueName() {
  return `_${Math.floor(((Math.random() + 1) / 2) * Number.MAX_SAFE_INTEGER)}`;
}

function invoke<T>(type: string, payload: any): Promise<T> {
  return new Promise((resolve, reject) => {
    const callback = generateUniqueName();
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
  static create() {
    if (window.external.invoke == null) {
      return new AppMock();
    }
    return new App();
  }

  init() {
    return invoke<{
      auOffsetsRepositoryUrl: string;
      gameSettingsList: readonly GameSettingsListItem[];
    }>('init', {});
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

  setOnChangeProcessStatus(listener: ((status: ProcessStatus) => void) | null) {
    window.onChangeProcessStatus = listener;
  }

  openBrowser(url: string) {
    return invoke<void>('open_browser', { url });
  }
}

export class AppMock extends App {
  async init() {
    return {
      auOffsetsRepositoryUrl: 'https://google.com',
      gameSettingsList: [...Array(10).keys()].map((x) => ({
        name: `Mock ${x + 1}`,
        gameSettings: x % 2 === 0 ? '' : null,
      })),
    };
  }
}
