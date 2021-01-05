import { CircularProgress } from '@material-ui/core';
import React, { useCallback, useEffect, useMemo, useState } from 'react';
import App from './App';
import MainContent from './MainContent';

export default function Main() {
  const app = useMemo(() => App.create(), []);
  const [state, setState] = useState({
    gameSettingsList: null as
      | readonly { name: string; gameSettings: Object | null }[]
      | null,
  });
  useEffect(() => {
    (async () => {
      const gameSettingsList = await app.gameSettingsList();
      setState({ gameSettingsList });
    })().catch(console.error);
  }, []);

  const onChangeLabel = useCallback(async (idx, value) => {
    await app.setGameSettingsName(idx, value);
  }, []);
  const onClickLoad = useCallback(async (idx) => {
    await app.loadMemoryFromFile(idx);
  }, []);
  const onClickSave = useCallback(async (idx) => {
    await app.saveMemoryToFile(idx);
    setState((old) => ({
      ...old,
      gameSettingsList:
        old.gameSettingsList?.map((x, i) =>
          i != idx || x.gameSettings != null ? x : { ...x, gameSettings: {} },
        ) ?? null,
    }));
  }, []);

  if (state.gameSettingsList == null) {
    return <CircularProgress />;
  }
  return (
    <MainContent
      gameSettingsList={state.gameSettingsList}
      onChangeLabel={onChangeLabel}
      onClickLoad={onClickLoad}
      onClickSave={onClickSave}
    />
  );
}
