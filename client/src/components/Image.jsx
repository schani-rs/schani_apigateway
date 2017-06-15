import React from 'react';
import PropTypes from 'prop-types';

const Image = ({ id, title, description, license }) => (
  <article key={id}>
    <div>Title: {title}</div>
    <div>Desc: {description}</div>
    <div>License: {license}</div>
  </article>
);

Image.propTypes = {
  id: PropTypes.number.isRequired,
  title: PropTypes.string.isRequired,
  description: PropTypes.string.isRequired,
  license: PropTypes.string.isRequired,
};

export default Image;
