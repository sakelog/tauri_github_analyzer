/* eslint no-param-reassign: 0 */
import { createSlice } from '@reduxjs/toolkit';

type ReducerState = {
  tmpPersonalToken: string;
  tokenSubmitted: boolean;
  errorMessage: string;
  countTokenSend: number;
};

const initialState: ReducerState = {
  tmpPersonalToken: '',
  tokenSubmitted: false,
  errorMessage: '',
  countTokenSend: 0,
};

export const mainSlice = createSlice({
  name: 'mainSlice',
  initialState,
  reducers: {
    setTmpPersonalToken: (state, action) => {
      state.tmpPersonalToken = action.payload;
    },
    setTokenSubmitted: (state, action) => {
      state.tokenSubmitted = action.payload;
    },
    setErrorMessage: (state, action) => {
      state.errorMessage = action.payload;
    },
    incrementCountTokenSend: (state) => {
      state.countTokenSend += 1;
    },
  },
});

export const {
  setTmpPersonalToken,
  setTokenSubmitted,
  setErrorMessage,
  incrementCountTokenSend,
} = mainSlice.actions;

export default mainSlice.reducer;
