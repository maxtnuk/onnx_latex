import { Html } from "@react-three/drei"
import styled from "styled-components";
import { useMemo } from "react";

const ShapeNameDiv = styled.div`
    border-radius: 25px;
    background: #73AD21;
    padding: 20px;
    color: #ffffff;
    font-size: 1rem;
    opacity: 0.8;
`;

function LayerShape(props) {
    const sizes = props.sizes;
    const position = props.position;
    const ratio = props.ratio;
    const term = 10;
    position[1] -= term / ratio
    let size_text = "";
    for (const i of sizes) {
        size_text += `${i}x`
    }
    const final_string = size_text.slice(0, -1);

    const l_shape = useMemo(() => {
        return (
            <mesh
                {...props}
                position={position}
            >
                <Html distanceFactor={300 / ratio}
                    center={true}
                >
                    <ShapeNameDiv>
                        <h1>{final_string}</h1>
                    </ShapeNameDiv>
                </Html>
            </mesh>
        )
    }, [sizes])

    return (
      <>
        {
            l_shape
        }
      </>
    )
}
export default LayerShape;