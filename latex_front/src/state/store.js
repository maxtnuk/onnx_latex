import layer_reducer from "./layer/reducer";
import { combineReducers } from "redux";
import camera_reducer from "./camera/reducer";
import model_reducer from "./model/reducer";

const rootReducer = combineReducers({
    // Define a top-level state field named `todos`, handled by `todosReducer`
    camera: camera_reducer,
    layer: layer_reducer,
    model: model_reducer
})

export default rootReducer;