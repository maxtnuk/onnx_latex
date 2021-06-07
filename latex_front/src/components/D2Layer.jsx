import * as THREE from 'three'
import styled from 'styled-components';
import { useDispatch, useSelector } from 'react-redux';
import { useEffect, useState, useRef, useMemo } from 'react';
import { choose_layer } from 'api/layer';
import { Box } from '@react-three/drei';
import { Html } from '@react-three/drei';
import { cloneDeep } from 'lodash';
import D2Lines from './D2Lines'
import LayerName from './LayerName';
import LayerShape from './LayerShape';

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

export const term = 20;
const mex_circles = 16;

export function calc_2d_width(ratio) {
    const result = circle_radius / ratio + 2 * term / ratio;
    return result
}
function circle_postions(base, many, scaled_radius) {
    const render_count = many > mex_circles ? mex_circles / 2 : many / 2 - 0.5;
    let right = many - 1;
    let left = 0;
    let result = []
    for (let i = 0; i <= render_count; i++) {
        //  if these are crossed, then break;
        if (right < left) {
            break;
        }
        // make right element
        let new_point = cloneDeep(base);
        new_point[1] += (i - render_count) * scaled_radius;
        result.push(new_point)

        // if it is not collapsed make left element
        if (right != left) {
            let new_point2 = cloneDeep(base);
            new_point2[1] += (render_count - i) * scaled_radius;
            result.push(new_point2)
        }
        right -= 1;
        left += 1;
    }
    return result;
}


function D2Layer(props) {
    const l_idx = props.l_idx
    const g_idx = props.g_idx
    const name_padding = props.name_padding;

    const ratio = props.ratio;
    // except batch size;
    const num_circles = props.size[1]
    const bs = props.size.map(x => x / ratio)
    const layer_info = props.layer;
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
    const model = useSelector(state => state.model)

    const new_point = cloneDeep(props.position);
    new_point[1] += ((num_circles > mex_circles ? mex_circles / 2 : num_circles / 2) + 1) * scaled_radius
    const new_point2 = cloneDeep(new_point)
    new_point2[1] = -new_point[1]

    useEffect(() => {
        if (layer.layer_idx == -1) {
            setActive(false)
        } else {
            setActive(layer.group_idx == g_idx && layer.layer_idx == l_idx)
        }

    }, [layer])
    // insert center
    // it is just test data
    const next_l_idx = layer_info.outputs[0];
    const is_next_2d = next_l_idx !== undefined ? model.symbol_map[next_l_idx].output_shape.length < 3 : false;

    // make multiple circles
    const { circles, lines } = useMemo(() => {
        let inner_circles = [];
        const currnet_circle_pos = circle_postions(props.position, num_circles, scaled_radius);
        // just insert test next d2 layer
        const next_layer = is_next_2d ? model.symbol_map[next_l_idx] : undefined;
        let next_pose = cloneDeep(props.position);
        next_pose[0] += 2 * term / ratio
        const next_circle_pos = is_next_2d ? circle_postions(next_pose, next_layer.output_shape[1], scaled_radius) : undefined;
        let lines = []
        for (const pos of currnet_circle_pos) {
            inner_circles.push(
                <mesh
                    {...props}
                    position={pos}
                >
                    <Html distanceFactor={scaled_radius * 100}
                        center={true}
                    >
                        <CircleDiv
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
                        />
                    </Html>
                </mesh>);
            if (is_next_2d && !props.is_last) {
                for (const nc of next_circle_pos) {
                    lines.push(
                        <D2Lines
                            from={pos}
                            to={nc}
                            color={color}
                        >

                        </D2Lines>
                    )
                }
            }
        }

        return {
            circles: inner_circles,
            lines: lines
        };
    }, [props.position])

    return (
        <group>
            {
                num_circles > mex_circles &&
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
            {
                lines
            }
            <LayerName
                name={props.layer.symbol}
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

export default D2Layer;