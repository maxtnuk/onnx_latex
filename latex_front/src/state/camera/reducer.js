import { stat } from "@nodelib/fs.stat"
import {INIT_CAMERA_VEC,CHANGE_CAMERA} from "./action"
import { ZOOM_CAMERA } from "./action"
import {RESET_CAMERA} from "./action"

export default function camera_reducer(state = INIT_CAMERA_VEC, action) {
    // The reducer normally looks at the action type field to decide what happens
    switch (action.type) {
        case CHANGE_CAMERA:
            return {
                ...state,
                x: action.x,
                y: action.y,
            }
        case ZOOM_CAMERA:
            return {
                ...state,
                zoom: action.zoom
            }
        case RESET_CAMERA:
            return {
                ...state,
                r: action.r
            }
        default:
            return state
    }
}