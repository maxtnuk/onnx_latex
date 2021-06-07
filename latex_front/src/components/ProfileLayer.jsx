import styled from "styled-components";
import { useMemo } from "react";
import LayerTable from "./LayerTable";
import { MathJaxContext } from "better-react-mathjax";
import { MathJax } from "better-react-mathjax";
import { useSelector } from "react-redux";
import { useCallback } from "react";
const RootDiv = styled.div`
    display: flex;
    width: 100%;
    flex-direction: column;
    justify-content: center;
`;

const LayerTitle = styled.div`
    font-size: 2.1em;
    display: flex;
    flex-direction: column;
`;
const LayerBasic = styled.div`
    width: 90%;
    border-style: solid;
    border-color: #000000;
    display: flex;
    flex-direction: column;
    font-size: 1rem;
    font-family: 'Roboto Condensed', sans-serif;
    margin-right: 5pt;
`;
const LayerAttribute = styled.div`
    display: flex;
    flex-direction: column;
    font-family: 'Roboto Condensed', sans-serif;
`;

const MathContainer = styled.div`
    display: flex;
    flex-direction: column;
    font-family: 'Roboto Condensed', sans-serif;
`;
const SubTitle = styled.div`
    padding: 5pt;
    font-size: 2em;
    font-family: 'Roboto Condensed', sans-serif;
`;

const config = {
    loader: { load: ["[tex]/html"] },
    tex: {
        packages: { "[+]": ["html"] },
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
function ProfileLayer(props) {
    const layer = props.layer;
    const shape_print = useCallback(
        (target) => {
            let size_text = "";
            for (const i of target) {
                size_text += `${i}x`
            }
            return size_text.slice(0, -1);
        },
        [layer],
    )
    
    const shape_string = shape_print(layer.output_shape)
    const model = useSelector(state => state.model);

    const {inputs,input_shape} = useMemo(() => {
        let inner_input_symbols = []
        let inner_shape=[]
        for (const i of layer.inputs) {
            const n_layer = model.symbol_map[i]
            inner_input_symbols.push(
                n_layer.symbol
            )
            inner_shape.push(
                shape_print(n_layer.output_shape)
            )
        }
        
        return {
            inputs: (
                <MathJaxContext>
                <MathJax hideUntilTypeset={"first"}>
                    {`\\(${inner_input_symbols.toString()}\\)`}
                </MathJax>
            </MathJaxContext>
            ),
            input_shape: (
                inner_shape.toString()
            )
        };
    }, [layer])
    const output = useMemo(() => {
        const output_idx = layer.outputs[0];
        const output_layer = model.symbol_map[output_idx];
        return (
            <MathJaxContext>
                <MathJax hideUntilTypeset={"first"}>
                    {`\\(${output_layer.symbol}\\)`}
                </MathJax>
            </MathJaxContext>
        )
    }, [layer])
    const basic_infos = {
        op_name: layer.op_name,
        output_shape: shape_string,
        inputs: inputs,
        input_shape: input_shape,
        output: output
    }
    return (
        <RootDiv>
            <LayerTitle>
                <MathJaxContext config={config}>
                    <MathJax hideUntilTypeset={"first"}>
                        {`\\[${layer.symbol}\\]`}
                    </MathJax>
                </MathJaxContext>
            </LayerTitle>
            <SubTitle>
                Layer Info
            </SubTitle>
            <LayerBasic>
                <LayerTable
                    tables={basic_infos}
                >

                </LayerTable>
            </LayerBasic>
            <LayerAttribute>

            </LayerAttribute>
            <SubTitle>
                순전파
            </SubTitle>
            <MathJaxContext config={config}>
                <MathJax hideUntilTypeset={"first"}>
                    {`\\[${layer.symbol}=${layer.forward_value}\\]`}
                </MathJax>
            </MathJaxContext>
            <SubTitle>
                역전파
            </SubTitle>
        </RootDiv>
    )
}
export default ProfileLayer;