import { configureStore } from '@reduxjs/toolkit';
import slice from 'redux/lib/slice';

export const store = configureStore({
  reducer: {
    mainState: slice,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
