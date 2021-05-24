import {INIT_CAMERA_VEC,CHANGE_CAMERA} from "./action"

export default function camera_reducer(state = INIT_CAMERA_VEC, action) {
    // The reducer normally looks at the action type field to decide what happens
    switch (action.type) {
        case CHANGE_CAMERA:
            return {
                ...state,
                x: action.x,
                y: action.y,
                z: action.z
            }
        default:
            return state
    }
}