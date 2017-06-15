import React from 'react';
import ReactDOM from 'react-dom';
import App from './containers/App';
import configureStore from './store';

const store = configureStore();

ReactDOM.render(
  <App store={store} />,
  /* eslint-disable */
  document.getElementById('App')
  /* eslint-enable */
);
