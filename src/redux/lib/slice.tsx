/* eslint no-param-reassign: 0 */
import { createSlice } from '@reduxjs/toolkit';

type ReducerState = {
  // isExistFile: boolean;
  tmpPersonalToken: string;
  tokenSubmitted: boolean;
};

const initialState: ReducerState = {
  // isExistFile: false,
  tmpPersonalToken: '',
  tokenSubmitted: false,
};

export const mainSlice = createSlice({
  name: 'mainSlice',
  initialState,
  reducers: {
    // setIsExistFile: (state, action) => {
    //   state.isExistFile = action.payload;
    // },
    setTmpPersonalToken: (state, action) => {
      state.tmpPersonalToken = action.payload;
    },
    setTokenSubmitted: (state, action) => {
      state.tokenSubmitted = action.payload;
    },
  },
});

export const { setTmpPersonalToken, setTokenSubmitted } =
  mainSlice.actions;

export default mainSlice.reducer;
