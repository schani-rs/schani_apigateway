import { handleActions } from 'redux-actions';
import { getAllImages, updateImage } from '../actions/images';

const defaultState = { images: [] };

export default handleActions({
  [getAllImages]: (state, action) => ({
    images: action.payload,
  }),
  [updateImage]: (state, action) => ({
    images: action.payload,
  }),
}, defaultState);
