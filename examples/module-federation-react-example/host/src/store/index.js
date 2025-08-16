import { configureStore } from '@reduxjs/toolkit';
import dashboardReducer from './slices/dashboardSlice.js';
import usersReducer from './slices/usersSlice.js';
import analyticsReducer from './slices/analyticsSlice.js';

export const store = configureStore({
  reducer: {
    dashboard: dashboardReducer,
    users: usersReducer,
    analytics: analyticsReducer,
  },
});
