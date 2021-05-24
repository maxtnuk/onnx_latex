import { CHOOSE_LAYER } from "state/layer/action";

export function choose_layer(group_idx,layer_idx) {
    return {
        type: CHOOSE_LAYER,
        group_idx: group_idx,
        layer_idx: layer_idx
    };
}
