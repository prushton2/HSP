import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter, Route, Routes } from "react-router";
import './index.css'
import App from './App.tsx'
import Admin from './pages/admin/Admin.tsx';
import Signup from './pages/signup/signup.tsx';
import Nursing from './pages/nursing/nursing.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/admin" element={<Admin />} />
        <Route path="/signup" element={<Signup />} />
        <Route path="/nursing" element={<Nursing />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
