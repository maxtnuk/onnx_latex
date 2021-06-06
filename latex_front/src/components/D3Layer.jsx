import React from "react";
import * as THREE from "three"
import { useEffect, useRef, useState } from "react"
import { useFrame } from '@react-three/fiber'
import { useDispatch, useSelector } from 'react-redux';
import { choose_layer } from "api/layer";
import { Box } from "@react-three/drei";
import { Html } from "@react-three/drei"
import { useThree } from "@react-three/fiber";
import LayerName from "./LayerName";
import { cloneDeep } from "lodash";
import LayerShape from "./LayerShape";

// draw 3d layer 
function D3Layer(props) {
  // get group idx, layer idx
  const l_idx = props.l_idx
  const g_idx = props.g_idx
  const name_padding= props.name_padding;
  // This reference will give us direct access to the THREE.Mesh object
  const mesh = useRef()
  // Set up state for the hovered and active state
  const [hovered, setHover] = useState(false)
  const [active, setActive] = useState(false)
  
  // unit for name padding
  const unit_padding=5;

  // mesh scale factor 
  const ratio=props.ratio;
  const bs=props.size.map(x => x /ratio)

  const color=props.color

  const lines=[bs[1], bs[2], bs[3]]

  const geometry = new THREE.BoxGeometry(lines[0], lines[1], lines[2]);
  // get cube edge 
  const edges = new THREE.EdgesGeometry(geometry);

  const dispatch = useDispatch()
  //  redux layer selection
  const layer = useSelector(state => state.layer)
// if layer selection is changed, set active value based on selection value
  useEffect(() => {
    if (layer.layer_idx == -1) {
      setActive(false)
    } else {
      setActive(layer.group_idx == g_idx && layer.layer_idx == l_idx)
    }

  }, [layer])
  // setActive(false)

  const html_object=useRef();
  const new_point= cloneDeep(props.position);
  new_point[1]+=lines[1]/2+1
  const new_point2=cloneDeep(new_point)
  new_point2[1]=-new_point[1]
  // Return the view, these are regular Threejs elements expressed in JSX
  return (
    <group>
       <mesh
      {...props}
      onPointerOver={(event) => {
        // if (!hovered){
        //   event.stopPropagation()
        // }
        setHover(true)
      }
      }
      onPointerOut={(event) => {
        setHover(false)
      }}
      onClick={(event) => {
        if (!layer.is_dragging){
          dispatch(choose_layer(g_idx, l_idx))
        }
      }}
    >
      {/* line edge mesh */}
      <lineSegments
        ref={mesh}
        geometry={edges}
        // scale={active ? 1.2 : 1}
      >
        {/* line edge color */}
        <lineBasicMaterial attach="material" color={hovered ? "blue" : "black"} />
      </lineSegments>
      {/* box mesh  */}
      <Box
        // scale={active ? 1.2 : 1}
        args={lines}>
        <meshPhongMaterial color={color} opacity={0.5} transparent={true}/>
      </Box>
    </mesh>
   <LayerName
          name={props.layer.op_name}
          color={color}
          sizes={props.size}
          ratio={ratio}
          position={
              new_point
          }
      />
    <LayerShape
      sizes={props.size}
      ratio={ratio}
      position={
          new_point2
      }
            />
    </group>
  )
}
export default D3Layer;