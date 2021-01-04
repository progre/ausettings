import 'regenerator-runtime/runtime.js';
import 'core-js/features/promise';

import ReactDOM from 'react-dom';
import CssBaseline from '@material-ui/core/CssBaseline';
import Main from './Main';

ReactDOM.render(
  <>
    <CssBaseline />
    <Main />
  </>,
  document.getElementById('root'),
);
