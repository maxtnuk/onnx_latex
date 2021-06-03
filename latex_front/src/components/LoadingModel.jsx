import styled from "styled-components";
import { keyframes } from "styled-components";
import spin_logo from "img/refresh-arrow.svg"

const LoadingInnerDiv = styled.div`
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    position: absolute;
    width: 400px;
    background-color: #ffffff;
    border: 2px solid #000;
    padding: 30px;
    text-align: center;
`;

const RotateAnim=keyframes`
    0% {
        transform: rotate(360deg)
    }
    25% {
        transform: rotate(270deg);
    }
    50% {
        transform: rotate(180deg);
    }
    75% {
        transform: rotate(90deg);
    }
    100% {
        transform: rotate(0deg);
    }
`;

const Spin=styled.img`
    width: 80px;
    height: 80px;
    animation: ${RotateAnim} 1.5s linear infinite;
`;

function LoadingModel(props){
    return (
        <LoadingInnerDiv
            {...props}
            >
            <h2>While Parsing Data</h2>
            <Spin
                src={spin_logo}
            />
        </LoadingInnerDiv>
    );
}

export default LoadingModel;