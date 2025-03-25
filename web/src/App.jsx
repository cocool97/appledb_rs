import './App.css';
import React from 'react';

import HomeRoute from './routes/HomeRoute';
import ModelPage from './routes/ModelPage';
import EntitlementsRoute from './routes/EntitlementsRoute';

import {
  BrowserRouter as Router,
  Route,
  Routes
} from "react-router-dom";
import CustomAppBar from './components/CustomAppBar';
import Stats from './routes/Stats';
import Diffing from './routes/Diffing';


const App = () => {
  return (
    <>
      <Router>
        <CustomAppBar />
        <Routes>
          <Route exact path="/" element={<HomeRoute />} />
          <Route exact path="/stats" element={<Stats />} />
          <Route exact path="/diff" element={<Diffing />} />
          <Route exact path="/model/:modelId" element={<ModelPage />} />
          <Route exact path="/model/:modelId/version/:versionId" element={<EntitlementsRoute />} />
          <Route path="*" element={<div>NOT IMPLEMENTED YET !</div>} />
        </Routes>
      </Router>
    </>
  );
}

export default App;