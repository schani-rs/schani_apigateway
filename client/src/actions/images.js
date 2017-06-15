import { createAction } from 'redux-actions';
import { getAll } from '../sources/images_source';

export const getAllImages = createAction('GET_ALL_IMAGES', () => getAll());
export const updateImage = createAction('UPDATE_IMAGE');
