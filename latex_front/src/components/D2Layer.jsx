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
// max circles 

const circle_radius = 10;

export const term = 3;

export function calc_2d_width(ratio) {
    return circle_radius / ratio + 2 * term;
}

function make_circle(props,radius,new_position){
    return (
        <mesh
            {...props}
            position={new_position}
        >
            <Html distanceFactor={radius * 100}
                center={true}
            >
                <CircleDiv />
            </Html>
        </mesh>
    )
}

function D2Layer(props) {
    const l_idx = props.l_idx
    const g_idx = props.g_idx
    const name_padding = props.name_padding;

    const ratio = props.ratio;
    // except batch size;
    const num_circles = props.size[1]
    const bs = props.size.map(x => x / ratio)
    // get radius 

    const color = props.color
    const scaled_radius = circle_radius / ratio;


    const [active, setActive] = useState(false)
    const [hovered, setHover] = useState(false)

    const mesh = useRef();
    const midle_box = [scaled_radius, scaled_radius, scaled_radius]

    const geometry = new THREE.BoxGeometry(midle_box[0], midle_box[1], midle_box[2]);
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
    // insert center

    // make multiple circles
    const circles = useMemo(() => {
        let inner_circles = [];
        const render_count = num_circles >20 ? 10: num_circles/2-0.5;
        let right=num_circles-1;
        let left =0;
        for (let i = 0; i <= render_count; i++) {
            //  if these are crossed, then break;
            if (right< left ){
                break; 
            }
            // make right element
            let new_point = cloneDeep(props.position);
            new_point[1] += (i - render_count) * scaled_radius;
            inner_circles.push(make_circle(props, scaled_radius, new_point));
            // if it is not collapsed make left element
            if (right!=left){
                let new_point2 = cloneDeep(props.position);
                new_point2[1] += (render_count-i) * scaled_radius;
                inner_circles.push(make_circle(props, scaled_radius, new_point2));
            }
            right-=1;
            left+=1;
        }
        return inner_circles;
    }, [props.position])

    return (
        <group>
            {
                num_circles > 20 &&
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
                        args={midle_box}>
                        <meshPhongMaterial color={color} opacity={0.5} transparent={true} />
                    </Box>
                </mesh>
            }
            {
                circles
            }
        </group>
    )
}

export default D2Layer;