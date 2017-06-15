import React from 'react';
import PropTypes from 'prop-types';
import { Provider } from 'react-redux';
import { Button } from 'react-bootstrap';
import Header from '../components/Header';
import DevTools from './DevTools';

const App = ({ store }) => (
  <Provider store={store} >
    <div>
      <Header />
      <div>
        This is Schani!<Button bsStyle="primary">Primary</Button>
      </div>
      <DevTools />
    </div>
  </Provider>
);


App.propTypes = {
  store: PropTypes.object.isRequired, // eslint-disable-line react/forbid-prop-types
};

export default App;
