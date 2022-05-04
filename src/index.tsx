import React from 'react';
import ReactDOM from 'react-dom/client';

// redux
import { store } from 'lib/redux/store';
import { Provider } from 'react-redux';

// Chakra
import { ChakraProvider } from '@chakra-ui/react';

// component
import App from './App';
import reportWebVitals from './reportWebVitals';

const container = document.getElementById('root');
const root = container && ReactDOM.createRoot(container);

root?.render(
  <React.StrictMode>
    <Provider store={store}>
      <ChakraProvider>
        <App />
      </ChakraProvider>
    </Provider>
  </React.StrictMode>
);

if (import.meta.env.DEV) {
  // eslint-disable-next-line no-console
  reportWebVitals(console.log);
}
