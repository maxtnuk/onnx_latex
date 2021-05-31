import { style } from "@material-ui/system";
import styled from "styled-components";

const NameContainer= styled.div`
    border-radius: 5px;
    padding: 1pt;
    opacity: 0.8;
`;


function LayerName(props){
    const name=props.name;
    const color=props.color;

    return (
        <NameContainer
            style={{
                ...style,
                background: color
            }
            }
        >
            <h1 style={{fontSize: "11px"}}>
                {name}
            </h1>
        </NameContainer>
    )
}

export default LayerName;