import { CircularProgress } from '@material-ui/core';
import React, { useCallback, useEffect, useMemo, useState } from 'react';
import App from './App';
import MainContent from './MainContent';

export default function Main() {
  const app = useMemo(() => App.create(), []);
  const [state, setState] = useState({
    labels: null as readonly string[] | null,
  });
  useEffect(() => {
    (async () => {
      const list = await app.gameSettingsList();
      setState({ labels: list.map((x) => x.name) });
    })().catch(console.error);
  }, []);

  const onChangeLabel = useCallback((idx, value) => {
    app.setGameSettingsName(idx, value);
  }, []);
  const onClickLoad = useCallback((idx) => {
    app.loadMemoryFromFile(idx);
  }, []);
  const onClickSave = useCallback((idx) => {
    app.saveMemoryToFile(idx);
  }, []);

  if (state.labels == null) {
    return <CircularProgress />;
  }
  return (
    <MainContent
      labels={state.labels}
      onChangeLabel={onChangeLabel}
      onClickLoad={onClickLoad}
      onClickSave={onClickSave}
    />
  );
}
