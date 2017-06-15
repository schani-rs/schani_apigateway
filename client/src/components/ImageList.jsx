import React from 'react';
import PropTypes from 'prop-types';
import { ListGroup, ListGroupItem } from 'react-bootstrap';
import Image from './Image';

const ImageList = ({ images }) => (
  <ListGroup>
    {images.map(i => (<ListGroupItem key={i.id}><Image {...i} /></ListGroupItem>))}
  </ListGroup>
);

ImageList.propTypes = {
  images: PropTypes.array.isRequired, // eslint-disable-line react/forbid-prop-types
};

export default ImageList;
