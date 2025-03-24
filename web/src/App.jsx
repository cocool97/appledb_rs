import './App.css';
import React from 'react';

import HomePage from './routes/HomePage';
import ModelPage from './routes/ModelPage';
import VersionPage from './routes/VersionPage';

import {
  BrowserRouter as Router,
  Route,
  Routes
} from "react-router-dom";

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