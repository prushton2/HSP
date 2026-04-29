import './index.css'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { ToastContainer } from './components/toast.tsx';
import App from './App.tsx';
import { Modal } from './components/Modal.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
    <Modal />
    <ToastContainer />
  </StrictMode>,
)
