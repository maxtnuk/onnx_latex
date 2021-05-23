import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import ModelPage from "./pages/ModelPage";
import MainPage from "./pages/MainPage";
// import {Mobile, PC} from "./components/Media"

function App() {
  return (
    <>
      <Router>
        <Switch>
          <Route path='/parse_model' component={ModelPage} /> 
          <Route path='/' component={MainPage} /> 
        </Switch>
      </Router>
    </>
  );
}
export default App;
