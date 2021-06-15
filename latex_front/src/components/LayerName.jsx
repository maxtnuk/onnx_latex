import { style } from "@material-ui/system";
import styled from "styled-components";
import { Html } from "@react-three/drei";
import { useMemo } from "react";
import { MathJaxContext } from "better-react-mathjax";
import { MathJax } from "better-react-mathjax";

const NameContainer = styled.div`
    border-radius: 25px;
    opacity: 0.8;
    font-size: 1rem;
    padding: 10px;
`;

const config = {
    loader: { load: ["[tex]/html", '[tex]/color'] },
    tex: {
        packages: { "[+]": ["html", 'color'] },
        autoload: { color: [] },
        inlineMath: [
            ["$", "$"],
            ["\\(", "\\)"]
        ],
        displayMath: [
            ["$$", "$$"],
            ["\\[", "\\]"]
        ]
    }
};


function LayerName(props) {
    const name = props.name;
    const color = props.color;
    const position = props.position
    const ratio = props.ratio;
    const term = 5;
    position[1] += term / ratio
    const l_name = useMemo(() => {
        return (
            <mesh
                {...props}
                position={position}
            >
                <Html distanceFactor={300 / ratio}
                    center={true}
                >
                    <NameContainer
                        style={{
                            ...style,
                            background: color
                        }
                        }
                    >
                        <MathJaxContext config={config}>
                            <MathJax dynamic>
                                {`\\({\\color{white} ${name}} \\)`}
                            </MathJax>
                        </MathJaxContext>
                    </NameContainer>
                </Html>
            </mesh>
        )
    }, [position])

    return (
        <>
            {
                l_name
            }
        </>
    )
}

export default LayerName;