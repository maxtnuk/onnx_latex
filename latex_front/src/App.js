import InitPage from "pages/InitPage";
import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import MainPage from "./pages/MainPage";
// import {Mobile, PC} from "./components/Media"

function App() {
  return (
    <>
      <Router>
        <Switch>
          <Route path='/' component={InitPage} /> 
          <Route path='/parse_model' component={MainPage} /> 
        </Switch>
      </Router>
    </>
  );
}
export default App;
