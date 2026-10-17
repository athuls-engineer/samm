import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import { api } from './api/client';
import './index.css';

// Intercept all HTTP/HTTPS links across the entire app and open them in the system browser
document.addEventListener('click', (e) => {
  const target = (e.target as HTMLElement)?.closest('a');
  if (target && target.href) {
    const href = target.href;
    if (href.startsWith('http://') || href.startsWith('https://')) {
      e.preventDefault();
      e.stopPropagation();
      api.openUrl(href).catch((err) => {
        console.error('Failed to open external link in browser:', err);
      });
    }
  }
}, true);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

