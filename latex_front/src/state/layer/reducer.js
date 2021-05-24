import {INIT_LAYER_CHOOSE,CHOOSE_LAYER} from "./action"

export default function layer_reducer(state = INIT_LAYER_CHOOSE, action) {
    // The reducer normally looks at the action type field to decide what happens
    switch (action.type) {
        case CHOOSE_LAYER:
            return {
                ...state,
                group_idx: action.group_idx,
                layer_idx: action.layer_idx
            }
        default:
            return state
    }
}