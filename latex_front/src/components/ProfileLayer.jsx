import styled from "styled-components";
import { useMemo } from "react";
import LayerTable from "./LayerTable";
import { MathJaxContext } from "better-react-mathjax";
import { MathJax } from "better-react-mathjax";
import { useSelector } from "react-redux";
import { useCallback } from "react";
import { useState } from "react";
import { useBackwardModel } from "api/rest_api";
import { useEffect } from "react";
import { makeStyles } from "@material-ui/styles";
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';

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
    width: 90%;
    justify-content: center;
    font-size: 0.9em;
`;
const SubTitle = styled.div`
    padding: 5pt;
    font-size: 2em;
    font-family: 'Roboto Condensed', sans-serif;
`;

//  backward styles 
const useStyles = makeStyles((theme) => ({
    root: {
        '& > *': {
            margin: 'ipt',
            width: '25ch',
        },
    },
}));

const BackwardForm = styled.form`
    display: flex;
    flex-direction: row;
`;

// mathjax config
const config = {
    loader: { load: ["[tex]/html", "output/svg"] },
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
    },
    svg: {
        linebreaks: { automatic: true }
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

    const model = useSelector(state => state.model);

    const [modelRequest, setmodelRequest] = useState({
        file: model.file,
        layer_node: layer.index,
        layer_idxs: {},
        weight_idxs: {},
        symbol: {
            symbol_map: model.symbol_map,
            senario: model.senario
        },
        depth: -1
    })

    const { error, during, res_backward } = useBackwardModel(modelRequest);

    useEffect(() => {
        console.log(error)
    }, [error])

    useEffect(() => {
        console.log(res_backward)
    }, [res_backward])

    const shape_string = shape_print(layer.output_shape)
    const [depth, setdepth] = useState(-1)
    const [l_idx, setl_idx] = useState(-1)
    const [w_idx, setw_idx] = useState(-1)
    const { inputs, input_shape } = useMemo(() => {
        let inner_input_symbols = []
        let inner_shape = []
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
            <>
                {
                    output_layer !== undefined &&
                    <MathJaxContext>
                        <MathJax hideUntilTypeset={"first"}>
                            {`\\(${output_layer.symbol}\\)`}
                        </MathJax>
                    </MathJaxContext>
                }
            </>
        )
    }, [layer])
    const basic_infos = {
        op_name: layer.op_name,
        output_shape: shape_string,
        inputs: inputs,
        input_shape: input_shape,
        output: output
    }

    const backward_styles = useStyles();

    // View 
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
            <MathContainer>
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
                <BackwardForm
                    className={backward_styles.root}
                    noValidate autoComplete="off"
                >
                    <TextField
                        type="text"
                        label="depth"
                        variant="outlined"
                        defaultValue={-1}
                        value={depth}
                        onChange={({ target: { value } }) => setdepth(value)}
                    />
                    <TextField
                        type="text"
                        label="layer index"
                        variant="outlined"
                        defaultValue={0}
                        value={l_idx}
                        onChange={({ target: { value } }) => setl_idx(value)}
                    />
                    <TextField
                        type="text"
                        label="weight index"
                        variant="outlined"
                        defaultValue={0}
                        value={w_idx}
                        onChange={({ target: { value } }) => setw_idx(value)}
                    />
                </BackwardForm>
                <Button
                    // send backward
                    onClick={() => {
                        const w_str = JSON.parse(w_idx);
                        const l_str = JSON.parse(l_idx);
                        const int_depth = JSON.parse(depth);
                        setmodelRequest({
                            ...modelRequest,
                            layer_idxs: l_str,
                            weight_idxs: w_str,
                            depth: int_depth
                        })
                    }}
                >
                    BackWard
                </Button>
            </MathContainer>
        </RootDiv>
    )
}
export default ProfileLayer;