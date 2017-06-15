import { createAction } from 'redux-actions';
import { getAll } from '../sources/images_source';

export const getAllImages = createAction('GET_ALL_IMAGES', () => {
  const val = getAll();
  console.log(val);
  return val;
});
export const updateImage = createAction('UPDATE_IMAGE');
