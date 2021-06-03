import * as THREE from 'three'
import styled from 'styled-components';
import { useDispatch, useSelector } from 'react-redux';
import { useEffect, useState, useRef, useMemo } from 'react';
import { choose_layer } from 'api/layer';
import { Box } from '@react-three/drei';
import { Html } from '@react-three/drei';
import { cloneDeep } from 'lodash';

const CircleDiv = styled.div`
    border: 1px solid #000;
    border-radius: 50%;
    width: 10px;
    height: 10px;
`;
const CircleContainer = styled.div`
    display: flex;
    flex-direction: column;
`;

export function calc_2d_width(num_circles, proper_height) {
    const radius = proper_height / num_circles / 2;
    return radius * 2+term;
}

export const term=6;

function D2Layer(props) {
    const l_idx = props.l_idx
    const g_idx = props.g_idx
    const name_padding = props.name_padding;

    const ratio = props.ratio;
    // except batch size;
    const num_circles = props.size[1]
    const bs = props.size.map(x => x)
    // get radius 
    const radius = bs[1] / num_circles / 2

    const color = props.color


    const [active, setActive] = useState(false)
    const [hovered, setHover] = useState(false)

    const mesh = useRef();

    // creaate long box for 2d size data 
    const box_lines = [radius * 2, bs[1], radius * 2]

    const geometry = new THREE.BoxGeometry(box_lines[0], box_lines[1], box_lines[2]);
    // get cube edge 
    const edges = new THREE.EdgesGeometry(geometry);

    const dispatch = useDispatch()
    const layer = useSelector(state => state.layer)
    useEffect(() => {
        if (layer.layer_idx == -1) {
            setActive(false)
        } else {
            setActive(layer.group_idx == g_idx && layer.layer_idx == l_idx)
        }

    }, [layer])

    // make multiple circles
    const circles = useMemo(() => {
        const center = num_circles / 2;
        let inner_circles=[];
        for (let i = 0; i < num_circles; i++) {
            let new_point = cloneDeep(props.position);
            new_point[1]+=(i-center*1)
            inner_circles.push(
                <mesh 
                    {...props}
                    position={new_point}
                >
                    <Html distanceFactor={100}
                    center={true}
                    >
                        <CircleDiv />
                    </Html>
                </mesh>
            )
        }
        return inner_circles;
    }, [props.position])

    return (
        <group>
            {/* <mesh
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
                    if (!layer.is_dragging) {
                        dispatch(choose_layer(g_idx, l_idx))
                    }
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
                    args={box_lines}>
                    <meshPhongMaterial color={color} opacity={0.5} transparent={true} />
                </Box>
            </mesh> */}
            {/* <mesh 
                    {...props}
                    position={[0,0,0]}
                >
                    <Html distanceFactor={100}
                    center={true}
                    >
                        <CircleDiv />
                    </Html>
            </mesh>
            <mesh 
                    {...props}
                    position={[0,2,0]}
                >
                    <Html distanceFactor={100}
                    center={true}
                    >
                        <CircleDiv />
                    </Html>
                </mesh>
                <mesh 
                    {...props}
                    position={[0,4,0]}
                >
                    <Html distanceFactor={100}
                    center={true}
                    >
                        <CircleDiv />
                    </Html>
                </mesh> */}
            {
                circles
            }
        </group>
    )
}

export default D2Layer;