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

function ZoomBar() {
    const direction = Direction.Up;
    const [value, setvalue] = useState(BASE)
    const dispatch = useDispatch(state => state.camera)

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
                    <div
                        onMouseDown={props.onMouseDown}
                        onTouchStart={props.onTouchStart}
                        style={{
                            ...props.style,
                            height: "100px",
                            display: "flex",
                            width: "100%"
                        }}
                    >
                        <div
                            ref={props.ref}
                            style={{
                                height: "100%",
                                width: "5px",
                                borderRadius: "4px",
                                background: getTrackBackground({
                                    values: [value],
                                    colors: ["#548BF4", "#ccc"],
                                    min: MIN,
                                    max: MAX,
                                    direction: direction
                                }),
                                alignSelf: "center"
                            }}
                        >
                            {children}
                        </div>
                    </div>
                )}
                renderThumb={({ props, isDragged }) => (
                    <div
                        {...props}
                        style={{
                            ...props.style,
                            height: "42px",
                            width: "42px",
                            borderRadius: "4px",
                            backgroundColor: "#FFF",
                            display: "flex",
                            justifyContent: "center",
                            alignItems: "center",
                            boxShadow: "0px 2px 6px #AAA"
                        }}
                    >
                        <div
                            style={{
                                height: "16px",
                                width: "5px",
                                backgroundColor: isDragged ? "#548BF4" : "#CCC"
                            }}
                        />
                    </div>
                )}
            />
        </ZoomDiv>
    )
}

export default ZoomBar;