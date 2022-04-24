import React from 'react';
import ReactDOM from 'react-dom/client';
import { env } from 'process';
import { ChakraProvider } from '@chakra-ui/react';

// component
import App from './App';
import reportWebVitals from './reportWebVitals';

const container = document.getElementById('root');
const root = container && ReactDOM.createRoot(container);

root?.render(
  <React.StrictMode>
    <ChakraProvider>
      <App />
    </ChakraProvider>
  </React.StrictMode>
);

if (env.NODE_ENV === 'development') {
  // eslint-disable-next-line no-console
  reportWebVitals(console.log);
}
