import D3Layer from "./D3Layer";
import { useRef } from "react";
import { useCallback } from "react";

function configure_layer(op){
    let layer_info={}
    let sizes=[]
    let color="black"
    let transparent=0.5
    
    switch (op.op_type) {
        case "cnn":
            sizes.push(op.channel)
            sizes.push(op.width)
            sizes.push(op.height)
            color="yellow"
            // transparent= base_transparent
            break;
        case "sum_pool":
            sizes.push(op.channel)
            sizes.push(op.width)
            sizes.push(op.height)
            color="red"
            // transparent= base_transparent
            break;
        case "max_pool":
            sizes.push(op.channel)
            sizes.push(op.width)
            sizes.push(op.height)
            color="green"
            // transparent= base_transparent
            break;
  
      default:
        break;
    } 
    layer_info.sizes=sizes
    layer_info.color=color
    layer_info.transparent=transparent
    return layer_info;
  }
export function get_group_width(layers,ratio){
    let sum_width=0;
    for (const i of layers){
        const layer_conf = configure_layer(i);
        sum_width+=layer_conf.sizes[0]/ratio;
    }
    return sum_width;
}

function GroupLayer(props) {
    const base_position=props.base;
    const group_idx=props.group_idx;
    const items = props.items;
    const layerItems = [];
    const ratio=props.ratio;

    let pos_sum=0;
    for (const i of items) {
        const layer_conf = configure_layer(i);
        const layer_width=layer_conf.sizes[0];
        let inner= <>
            <D3Layer
                size={layer_conf.sizes}
                color={layer_conf.color}
                transparent={layer_conf.transparent}
                ratio={ratio}
                position={[base_position+pos_sum+layer_width/ratio/2, 0, 0]}
                l_idx={i.layer_num}
                g_idx={group_idx}
            />
        </>
        layerItems.push(inner);
        console.log(layer_conf)
        console.log(pos_sum)
        pos_sum+=(layer_width/ratio);
    }
    return (
        <>
            {layerItems}
        </>
    )
}
export default GroupLayer;