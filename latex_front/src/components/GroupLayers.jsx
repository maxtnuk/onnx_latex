import D3Layer from "./D3Layer";
import { useRef, useMemo} from "react";
import { useCallback } from "react";

// configure layers base on operation type
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
// get total group layer x size (width) based on layers
export function get_group_width(layers,ratio){
    let sum_width=0;
    for (const i of layers){
        const layer_conf = configure_layer(i);
        sum_width+=layer_conf.sizes[0]/ratio;
    }
    return sum_width;
}

function compare_name_width(width,word){
    // each char size
    const each_char_size=2;
    // console.log(`word: ${word},width: ${each_char_size*word.length}, word lenght ${width}`)
    if (each_char_size*word.length>width){
        // name overflow
        return true;
    }else{
        return false;
    }
}

// draw multiple layers side by side
function GroupLayer(props) {
    const base_position=props.base;
    const group_idx=props.group_idx;
    const items = props.items;
    const ratio=props.ratio;

    const name_axis=10;

    const layers=useMemo(()=>{
        const layerItems = [];
        let pos_sum=0;
        // for name padding
        let each_namepadding=0;
        let continue_increase=false;
        for (const i of items) {
            const layer_conf = configure_layer(i);
            // get width without ratio 
            const layer_width=layer_conf.sizes[0];
            // TODO make 2d layer 
            if (continue_increase){
                // if true increase count
                if (compare_name_width(layer_width,i.op_type)){
                    each_namepadding+=1;
                }else{
                    // else reset count
                    each_namepadding=0;
                    continue_increase=false;
                }
            }else{
                each_namepadding+=1;
                continue_increase=true;
            }
            let inner= <>
                <D3Layer
                    size={layer_conf.sizes}
                    color={layer_conf.color}
                    transparent={layer_conf.transparent}
                    layer={i}
                    name_padding={each_namepadding}
                    ratio={ratio}
                    position={[base_position+pos_sum+layer_width/ratio/2, 0, 0]}
                    l_idx={i.layer_num}
                    g_idx={group_idx}
                />
            </>
            layerItems.push(inner);
            pos_sum+=(layer_width/ratio);
        }
        return layerItems
    },[items]);
   
    return (
        <>
            {layers}
        </>
    )
}
export default GroupLayer;