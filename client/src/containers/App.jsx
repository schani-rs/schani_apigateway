import React from 'react';
import { Button } from 'react-bootstrap';
import Header from '../components/Header';

class App extends React.Component {
  render() {
    return (
      <div>
        <Header />
        <div>
          This is Schani!<Button bsStyle="primary">Primary</Button>
        </div>
      </div>
    );
  }
}

export default App;
