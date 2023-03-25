import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import createCache from '@emotion/cache';
import { CacheProvider } from '@emotion/react';
import { CssBaseline } from '@mui/material';

import type { AppProps } from 'next/app';

const muiCache = createCache({
  key: 'mui',
  prepend: true,
});

export default function App({ Component, pageProps }: AppProps) {
  return (
    <CacheProvider value={muiCache}>
      <CssBaseline />
      <Component {...pageProps} />
    </CacheProvider>
  );
}
