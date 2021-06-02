import { SET_MODEL } from "state/model/action";

export function set_model(senario,symbol_map){
    return {
        type: SET_MODEL,
        senario: senario,
        symbol_map: symbol_map
    };
}