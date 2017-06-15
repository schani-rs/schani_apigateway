import React from 'react';
import PropTypes from 'prop-types';
import { Provider, connect } from 'react-redux';
import { Button } from 'react-bootstrap';
import Header from '../components/Header';
import DevTools from './DevTools';
import ImageList from '../components/ImageList';
import { getAllImages } from '../actions/images';

const App = ({ store, images, loadImages }) => (
  <Provider store={store} >
    <div>
      <Header />
      <div>
        This is Schani!
        <Button onClick={loadImages} bsStyle="primary">Primary</Button>
      </div>
      <ImageList images={images} />
      <DevTools />
    </div>
  </Provider>
);

App.propTypes = {
  store: PropTypes.object.isRequired, // eslint-disable-line react/forbid-prop-types
  images: PropTypes.array.isRequired, // eslint-disable-line react/forbid-prop-types
  loadImages: PropTypes.func.isRequired,
};

const mapStateToProps = state => ({
  images: state.images.images,
});

const mapDispatchToProps = dispatch => ({
  loadImages: () => dispatch(getAllImages()),
});

export default connect(mapStateToProps, mapDispatchToProps)(App);
