import layer_reducer from "./layer/reducer";
import { combineReducers } from "redux";
import { createStore } from "redux";

const rootReducer = combineReducers({
    // Define a top-level state field named `todos`, handled by `todosReducer`
    layer: layer_reducer,
})

export default rootReducer;