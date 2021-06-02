import { INIT_MODEL, SET_MODEL } from "./action"

export default function model_reducer(state = INIT_MODEL, action) {
    // The reducer normally looks at the action type field to decide what happens
    switch (action.type) {
        case SET_MODEL:
            return {
                ...state,
                senario: action.senario,
                symbol_map: action.symbol_map
            }
        default:
            return state
    }
}