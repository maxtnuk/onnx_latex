import * as THREE from 'three';
import { useMemo } from "react";
import { useCallback } from 'react';
import { Line } from '@react-three/drei';

function D2Lines(props){

    const color = props.color;
    const from = props.from;
    const to = props.to;

    const array_to_vector = (pos) => {
        return new THREE.Vector3(pos[0], pos[1], pos[2])
    }

    const vectors= useMemo(()=>{
        return [
            array_to_vector(from),
            array_to_vector(to)
        ]
    },[from,to])


    return (
        <Line
        points={[from,to]}       
        color={color}                   
        lineWidth={1}                  
        dashed={false}                  
      />
    )
}
export default D2Lines;