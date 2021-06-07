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
import { useEffect, useState,useMemo } from "react";
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
import { Html } from "@react-three/drei"
import mock_d2 from "mock/D2LayerMock";
import ProfileLayer from "components/ProfileLayer";
import CloseIcon from '@material-ui/icons/Close';

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

export const DragType={
  MainPage: "main_page"
}
// check each layer is end of group
function check_end_group(op_name){
  const end_list=["pool","clip","sigmoid"];
  const l_str=op_name.toLowerCase();
  return end_list.some((e)=>{return l_str.includes(e);})
}

function configure_model(models){
    const senario=models.senario;
    const symbol_map=models.symbol_map;
    let group_count=0;
    let layer_count=0;
    let groups=[]
    let layers=[]
    console.log(senario)
    for (const [index,i] of senario.entries()){
      const each_layer=symbol_map[i];
      layers.push({
        ...each_layer,
        layer_num: layer_count,
      });
      layer_count+=1;
      // if it is end layer, then create new group
      if (check_end_group(each_layer.op_name) || index==senario.length-1){
        console.log(index)
        let group= {
          group: group_count,
          layers: layers
        }
        groups.push(group);
        layers=[];
        layer_count=0;
        group_count+=1;
      }
    }
    return groups;
}
export const term = 30;
function MainPage() {
  // give mock data for test

  const [layerInfo, setlayerInfo] = useState({
    group_idx: -1,
    layer_num: -1,
  });

  const ReduxBridge = useContextBridge(ReactReduxContext);
  const model_form = useSelector(state => state.model);
  const layer = useSelector((state) => state.layer);
  const dispatch = useDispatch();

  // first memo recieved data
  let group_data = useMemo(
    () =>{
      return configure_model(model_form);
      // just for test 
      //return mock_d2
    },
  [model_form])

  // state side menu open 
  const [open, setopen] = useState(false);
  // if layer seleted, open side menu and print layer information 
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

  let before_content=0;
  // visual scale
  const ratio=5;
  // generate groups base on gorup_data 
  const groups=useMemo(() => {
    let group_layers = []
    for (const [i,g] of group_data.entries()){
      let group_width=get_group_width(g.layers,ratio);
      group_layers.push(
        <GroupLayer
          key={`group_${i}`}
          items={g.layers}
          ratio={ratio}
          group_idx={g.group}
          base={before_content}
        />
      )
      if (i!==group_data.length-1){
        const from=before_content+group_width;
        const dir = new THREE.Vector3(1,0,0);
        const origin=new THREE.Vector3(from,0,0);
        const color=0x000000;
        
        group_layers.push(<arrowHelper 
          args={[dir, origin, term/ratio, color]}>
          </arrowHelper>)
      }
      before_content+=(group_width+term/ratio)
    }
    return group_layers
  }, [group_data])
 

return (
  <>
    <MainaContainer>
      {/* layer graphic part */}
      <VisContainer>
        <Canvas camera={{ position: [0, 0, 20] }}>
          <ReduxBridge>
            <Controls />
            <ambientLight />
            <pointLight position={[10, 10, 10]} />
            
              <group>
              {
                groups
              }
              </group>
          </ReduxBridge>
        </Canvas>
      </VisContainer>
      {/* side menu part  */}
      <SidePane
        open={open}
        width={30}
        onClose={() => {}}
        hideBackdrop={true}
        disableBackdropClick={false}
        disableRestoreFocus={false}
      >
        <SideComponent>
          <CloseIcon
            style={{
              fontSize: "3em"
            }}
            onClick={() => {
              dispatch(choose_layer(-1, -1));
              setopen(false);
            }}
          />
          <ProfileLayer
            layer={layerInfo}
          >

          </ProfileLayer>
        </SideComponent>
      </SidePane>
      {/* controller part */}
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
        <Button variant="contained" color={"primary"}
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
