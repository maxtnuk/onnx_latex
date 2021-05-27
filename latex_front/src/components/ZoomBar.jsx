import { useState } from "react";
import { Range } from "react-range";
import { Direction } from "react-range";
import { getTrackBackground } from "react-range";
import styled from "styled-components";
import { useDispatch } from "react-redux";
import { zoom_camera } from "api/camera";

const STEP = 0.1;
const MIN = 0;
const MAX = 100;
export const BASE = 50;


const ZoomDiv = styled.div`
    display: flex;
    justifyContent: center;
    flexWrap: wrap;
    margin: 2em;
    padding: 10px;
`;

const ZoomTrackContainer = styled.div`
    height: 36px;
    display: flex;
    width: 100%;
`;

const ZoomThumb = styled.div`
    height: 42px;
    width: 42px;
    borderRadius: 4px;
    backgroundColor: #FFF;
    display: flex;
    justifyContent: center;
    alignItems: center;
    boxShadow: 0px 2px 6px #AAA;
`;



function ZoomBar() {
    const direction = Direction.Up;
    const [value, setvalue] = useState(BASE)
    const dispatch = useDispatch(state => state.camera)

    const ZoomTrack = styled.div`
        height: 100px;
        width: 10px;
        borderRadius: 4px;
        background: ${getTrackBackground({
            values: [value],
            colors: ["#548BF4", "#ccc"],
            min: MIN,
            max: MAX,
            direction: direction
        })};
        alignSelf: center; 
    `;
    return (
        <ZoomDiv>
            <Range
                values={[value]}
                step={STEP}
                min={MIN}
                max={MAX}
                direction={direction}
                onChange={(values) => {
                    setvalue(values[0])
                    dispatch(zoom_camera(values[0]))
                }}
                renderTrack={({ props, children }) => (
                    <ZoomTrackContainer>
                        <ZoomTrack>
                            {children}
                        </ZoomTrack>
                    </ZoomTrackContainer>
                )}
                renderThumb={({ props, isDragged }) => (
                    <ZoomThumb>
                        <div
                            style={{
                                height: "16px",
                                width: "10px",
                                backgroundColor: isDragged ? "#548BF4" : "#CCC"
                            }}
                        />
                    </ZoomThumb>
                )}
            />
        </ZoomDiv>
    )
}

export default ZoomBar;