import merge from 'lodash/merge';

export default (state = { images: [] }, action) => merge({}, state, action);
