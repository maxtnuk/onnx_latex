import { entries } from "lodash";
import styled from "styled-components";
const EachRow = styled.div`
    display: flex;
    flex-direction: row;
    font-size: 1em;
    font-family: 'Roboto Condensed', sans-serif;
`;
const EachName =styled.div`
    padding: 10pt;
    font-size: 1rem;
    text-align: center;
`;
const EachValue = styled.div`
    padding : 10pt;
    font-size: 1rem;
    text-align: center;
`;

function LayerTable(props){
    const tables = props.tables;
    let list_content=[]
    for (const [key,value] of Object.entries(tables)){
        list_content.push(
            <EachRow>
                <EachName>
                    {key}
                </EachName>
                <EachValue>
                    {value}
                </EachValue>
            </EachRow>
        )
    }
    return (
        <>
        {
            list_content
        }
        </>
    )
}

export default LayerTable;