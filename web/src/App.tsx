import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Dashboard from './pages/Dashboard';
import DockerManager from './pages/DockerManager';
import './App.css';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/docker/:uuid" element={<DockerManager />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;