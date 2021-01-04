import {
  Button,
  Container,
  TextField,
  Typography,
  makeStyles,
} from '@material-ui/core';
import React, { useCallback, FocusEvent } from 'react';

const useStyles = makeStyles({
  root: {
    marginTop: '16px',
  },
  listContainer: {
    padding: 0,
    margin: '32px 0',
  },
  listItem: {
    display: 'flex',
    alignItems: 'center',
    margin: 10,
  },
  text: {
    flexGrow: 1,
  },
  button: {
    marginLeft: 10,
  },
});

function ListItem(props: {
  index: number;
  label: string;
  onChangeLabel(index: number, value: string): void;
  onClickSave(index: number): void;
  onClickLoad(index: number): void;
}) {
  const onChangeLabel = useCallback(
    (e: FocusEvent) => {
      const target = e.target as HTMLInputElement;
      props.onChangeLabel(props.index, target.value);
    },
    [props.index],
  );
  const onClickSave = useCallback(() => props.onClickSave(props.index), [
    props.index,
  ]);
  const onClickLoad = useCallback(() => props.onClickLoad(props.index), [
    props.index,
  ]);
  const classes = useStyles();
  return (
    <>
      <TextField
        className={classes.text}
        defaultValue={props.label}
        onBlur={onChangeLabel}
      />
      <Button
        className={classes.button}
        color="secondary"
        variant="outlined"
        onClick={onClickSave}
      >
        Save
      </Button>
      <Button
        className={classes.button}
        color="primary"
        variant="outlined"
        onClick={onClickLoad}
      >
        Load
      </Button>
    </>
  );
}

export interface Props {
  labels: readonly string[];
  onChangeLabel(index: number, value: string): void;
  onClickSave(index: number): void;
  onClickLoad(index: number): void;
}

export default function MainContent(props: Props) {
  const classes = useStyles();
  return (
    <Container className={classes.root}>
      <Typography>
        <strong>⚠️Important</strong>:<br />
        After LOAD, <b>change any of the settings in the game</b> to apply the
        settings to other players.
      </Typography>
      <ul className={classes.listContainer}>
        {props.labels.map((x, i) => (
          <li className={classes.listItem}>
            <ListItem
              index={i}
              label={x}
              onChangeLabel={props.onChangeLabel}
              onClickSave={props.onClickSave}
              onClickLoad={props.onClickLoad}
            />
          </li>
        ))}
      </ul>
    </Container>
  );
}
