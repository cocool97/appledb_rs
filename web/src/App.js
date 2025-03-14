import './App.css';
import React from 'react';
import HomePage from './Pages/HomePage';
import ModelPage from './Pages/ModelPage';


import {
  BrowserRouter as Router,
  Route,
  Routes
} from "react-router-dom";
import VersionPage from './Pages/VersionPage';

const App = () => {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/model/:modelId" element={<ModelPage />} />
        <Route path="/model/:modelId/version/:versionId" element={<VersionPage />} />
      </Routes>
    </Router>
  );
}

export default App;