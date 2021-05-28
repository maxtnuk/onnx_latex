import * as THREE from "three";
// import { SVGRenderer } from '../resource/SVGRenderer'
import React from "react";
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
import mock_groups from "mock/GroupLayerMock";
import { choose_layer } from "api/layer";
import { Joystick } from "react-joystick-component";
import { camera_vec } from "api/camera";
import ZoomBar from "components/ZoomBar";
import { Button } from "@material-ui/core";
import { style } from "@material-ui/system";
import { reset_camera } from "api/camera";
import GroupLayer from "components/GroupLayers";
import { get_group_width } from "components/GroupLayers";
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
  border-color: #12b65c;
  bottom: 0;
  left: 0;
  background: white;
  display: flex;
  flex-direction: column-reverse;
  margin: 10px;
  pointer-events: null;
`;

function MainPage() {
  let group_data = mock_groups;

  const [layerInfo, setlayerInfo] = useState({
    group_idx: -1,
    layer_num: -1,
    channel: -1,
    width: -1,
    height: -1,
  });

  const ReduxBridge = useContextBridge(ReactReduxContext);
  const layer = useSelector((state) => state.layer);
  const dispatch = useDispatch();

  const [open, setopen] = useState(false);
  useEffect(() => {
    const l_num = layer.layer_idx;
    const g_num =layer.group_idx;
    if (l_num != -1) {
      const layer_data=(group_data[g_num].layers)[l_num];
      setlayerInfo({
        ...layer_data,
        group_idx: layer.group_idx,
        layer_num: l_num,
      });
      setopen(true);
    }
  }, [layer]);

  let group_layers = []
  let before_content=0;
  const ratio=10;
  const term = 3;
  for (const g of group_data){
    let group_width=get_group_width(g.layers,ratio);
    console.log(g);
    group_layers.push(
      <GroupLayer
        items={g.layers}
        ratio={ratio}
        group_idx={g.group}
        base={before_content}
      />
    )
    console.log(before_content)
    before_content+=(group_width+term)
  }

return (
  <>
    <MainaContainer>
      <VisContainer>
        <Canvas camera={{ position: [0, 0, 20] }}>
          <ReduxBridge>
            <Controls />
            <ambientLight />
            <pointLight position={[10, 10, 10]} />
            {/* <GroupLayer
              items={group_data[0].layers}
              ratio={ratio}
              group_idx={0}
              base={0}/>
            <GroupLayer
              items={group_data[1].layers}
              ratio={ratio}
              group_idx={1}
              base={20}/> */}
              <>
              {
                group_layers
              }
              </>
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
            dispatch(camera_vec(x, y));
          }}
          stop={() => {
            dispatch(camera_vec(0, 0));
          }}
        ></Joystick>
       {/* <ZoomBar/>  */}
        <Button variant="contained" color="yellow"
          onClick={(event) => {
            dispatch(reset_camera(true))
          }}
          style={{...style,margin: "10px"}}
        >
          Reset
        </Button>
      </MenuController>
    </MainaContainer>
  </>
);
}
export default MainPage;
