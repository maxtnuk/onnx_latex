import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import ModelPage from "./pages/ModelPage";
import MainPage from "./pages/MainPage";
import {Mobile, PC} from "./components/Media"

function App() {
  return (
    <>
      <Router>
        <Switch>
          <Route path='/' component={MainPage} /> 
          <Route path='/parse_model' component={ModelPage} /> 
        </Switch>
      </Router>
      {/* <div>
      <Mobile>
        <div className="mobile_container">
        이건 모바일 !!
        </div>
      </Mobile>
    </div>

    <div className="pc_container">
      <PC >
        요건 PC !!!
      </PC>
    </div> */}
    </>
  );
}
export default App;
