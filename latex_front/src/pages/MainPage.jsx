import * as THREE from "three";
// import { SVGRenderer } from '../resource/SVGRenderer'
import React from "react";
import LayerImage from "components/LayerImage";
import { MathJaxContext } from "better-react-mathjax";
import { MathJax } from "better-react-mathjax";
import { Canvas, useFrame } from "@react-three/fiber";
import styled from "styled-components";
import Controls from "components/Controls";
import { useContextBridge } from "@react-three/drei";
import { ReactReduxContext } from "react-redux";
import { SidePane } from "react-side-pane";
import { useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { useDispatch } from "react-redux";
import mock_layer from "./LayersMock";
import { choose_layer } from "api/layer";
import { Joystick } from "react-joystick-component";
import { camera_vec } from "api/camera";

const MainaContainer = styled.div`
  overflow: hidden;
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
`;

const VisContainer = styled.div`
  width: 100%;
  height: 100%;
`;

const SideComponent = styled.div`
  display: block;
`;
const MenuController = styled.div`
  position: absolute;
  border-style: solid;
  border-color: black;
  bottom: 0;
  left: 0;
  background: white;
  display: flex;
  flex-direction: column-reverse;
  margin: 10px;
`;

function MainPage() {
  let items = [];
  let mock_data = mock_layer;
  const ratio = 10;

  for (const i of mock_data) {
    items.push(
      <>
        <LayerImage
          d3s={[i.channel / ratio, i.width / ratio, i.height / ratio]}
          position={[i.layer_num * 3, 0, 0]}
          l_idx={i.layer_num}
          g_idx={0}
        />
      </>
    );
  }
  const [layerInfo, setlayerInfo] = useState({
    layer_num: -1,
    channel: -1,
    width: -1,
    height: -1,
  });

  const ReduxBridge = useContextBridge(ReactReduxContext);
  const layer = useSelector((state) => state.layer);
  const dispatch = useDispatch();

  const [open, setopen] = useState(false);
  const [camerapos, setcamerapos] = useState([0, 0, 0]);

  useEffect(() => {
    let l_num = layer.layer_idx;
    if (l_num != -1) {
      setlayerInfo(mock_data[l_num]);
      setopen(true);
    }
  }, [layer]);

  return (
    <>
      <MainaContainer>
        <VisContainer>
          <Canvas camera={{ position: [0, 0, 20] }}>
            <ReduxBridge>
              <Controls />
              <ambientLight />
              <pointLight position={[10, 10, 10]} />
              {/* s<Plane args={[10, 10]} color='black' /> */}
              {items}
            </ReduxBridge>
          </Canvas>
        </VisContainer>
        <SidePane
          open={open}
          width={30}
          onClose={() => {}}
          hideBackdrop={true}
          disableBackdropClick={false}
          disableRestoreFocus={false}
        >
          <SideComponent>
            <button
              onClick={() => {
                dispatch(choose_layer(-1, -1));
                setopen(false);
              }}
            >
              Close
            </button>
            <h1>Layer num: {layerInfo.layer_num}</h1>
            <h2>layer channel: {layerInfo.channel}</h2>
            <h2>layer width: {layerInfo.width}</h2>
            <h2>layer height: {layerInfo.height}</h2>
          </SideComponent>
        </SidePane>
        <MenuController>
          <Joystick
            size={100}
            baseColor="red"
            stickColor="blue"
            move={(mv) => {
              const x = mv.x;
              const y = mv.y;
              dispatch(camera_vec(x, y, 0));
            }}
            stop={() => {
              dispatch(camera_vec(0, 0, 0));
            }}
          ></Joystick>
        </MenuController>
      </MainaContainer>
    </>
  );
}
export default MainPage;
