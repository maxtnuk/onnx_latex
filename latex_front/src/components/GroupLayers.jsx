import D3Layer from "./D3Layer";
import { useRef, useMemo} from "react";
import { useCallback } from "react";
import D2Layer from "./D2Layer";
import { D2LayerWidth } from "./D2Layer";
import { calc_2d_width } from "./D2Layer";

// configure layers base on operation type
function configure_layer(op){
    let layer_info={}
    let sizes=op.output_shape
    let color="black"
    let transparent=0.5
    
    switch (op.op_name) {
        case "cnn":
            color="yellow"
            // transparent= base_transparent
            break;
        case "sum_pool":
            color="red"
            // transparent= base_transparent
            break;
        case "max_pool":
            color="green"
            // transparent= base_transparent
            break;
        case "Gemm":
            color="purple"
            break;
        case "Clip":
            color="#8b1c1c"
            break;
        case "Sigmoid":
            color="#1c8b60"
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
        const layer_width=get_width_with_size(layer_conf.sizes,ratio);
        sum_width+=layer_width;
        
    }
    return sum_width;
}

function get_width_with_size(sizes,ratio){
    // 2d data
    if (sizes.length<=2){
        // if you want, you can add ratio
        return calc_2d_width(ratio);
    }else if(sizes.length>=3){
        return sizes[sizes.length-1-2]/ratio
    }
}

function compare_name_width(width,word){
    // each char size
    const each_char_size=3;
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
        for (const [index,i] of items.entries()) {
            const layer_conf = configure_layer(i);
            // get width without ratio 
            const layer_width=get_width_with_size(layer_conf.sizes,ratio);
            // TODO make 2d layer 
            if (continue_increase){
                // if true increase count
                if (compare_name_width(layer_width,i.op_name)){
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
            const is_last=index == items.length-1
            // 2d layer
            switch(layer_conf.sizes.length){
                // 2d layer
                case 2:
                    layerItems.push(<D2Layer
                        is_last={is_last}
                        key={`layer_${i.layer_num}`}
                        size={layer_conf.sizes}
                        color={layer_conf.color}
                        transparent={layer_conf.transparent}
                        layer={i}
                        name_padding={each_namepadding}
                        ratio={ratio}
                        position={[base_position+pos_sum+layer_width/2, 0, 0]}
                        l_idx={i.layer_num}
                        g_idx={group_idx}
                    />)
                    break;
                case 3: 
                    break;
                // image network
                case 4:
                    layerItems.push(<D3Layer
                        is_last={is_last}
                        key={`layer_${i.layer_num}`}
                        size={layer_conf.sizes}
                        color={layer_conf.color}
                        transparent={layer_conf.transparent}
                        layer={i}
                        name_padding={each_namepadding}
                        ratio={ratio}
                        position={[base_position+pos_sum+layer_width/2, 0, 0]}
                        l_idx={i.layer_num}
                        g_idx={group_idx}
                    />)
                    break;
                default:
                    break;
            }
            pos_sum+=layer_width;
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