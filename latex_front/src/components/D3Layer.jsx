import React from "react";
import * as THREE from "three"
import { useEffect, useRef, useState } from "react"
import { useFrame } from '@react-three/fiber'
import { useDispatch, useSelector } from 'react-redux';
import { choose_layer } from "api/layer";
import { Box } from "@react-three/drei";

function D3Layer(props) {
  const l_idx = props.l_idx
  const g_idx = props.g_idx
  // This reference will give us direct access to the THREE.Mesh object
  const mesh = useRef()
  // Set up state for the hovered and active state
  const [hovered, setHover] = useState(false)
  const [active, setActive] = useState(false)
  const ratio=props.ratio;
  const bs=props.size.map(x => x /ratio)

  const color=props.color

  const geometry = new THREE.BoxGeometry(bs[0], bs[1], bs[2]);

  const edges = new THREE.EdgesGeometry(geometry);

  const dispatch = useDispatch()
  //  get value
  const layer = useSelector(state => state.layer)


  useEffect(() => {
    if (layer.layer_idx == -1) {
      setActive(false)
    } else {
      setActive(layer.group_idx == g_idx && layer.layer_idx == l_idx)
    }

  }, [layer])
  // setActive(false)

  // Return the view, these are regular Threejs elements expressed in JSX
  return (
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
        dispatch(choose_layer(g_idx, l_idx))
      }}
    >
      <lineSegments
        ref={mesh}
        geometry={edges}
        // scale={active ? 1.2 : 1}
      >
        <lineBasicMaterial attach="material" color={hovered ? "blue" : "black"} />
      </lineSegments>
      <Box
        // scale={active ? 1.2 : 1}
        args={bs}>
        <meshPhongMaterial color={color} opacity={0.5} transparent={true}/>
      </Box>
    </mesh>
  )
}
export default D3Layer;