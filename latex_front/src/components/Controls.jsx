import * as React from 'react';
import { extend, useThree, useFrame } from '@react-three/fiber';
import { TrackballControls } from 'three/examples/jsm/controls/TrackballControls';
import * as THREE from 'three';
import { useSelector, useDispatch } from 'react-redux'
import { useEffect, useState } from 'react';
import { MapControls } from '@react-three/drei'
import { zoom_camera } from 'api/camera';
import { BASE } from './ZoomBar';
import {reset_camera } from 'api/camera'

// extend THREE to include TrackballControls
extend({ TrackballControls });

// key code constants
const ALT_KEY = 18;
const CTRL_KEY = 17;
const CMD_KEY = 91;

function pancamera(self_scope,x,y) {
  const eye = new THREE.Vector3();
  const pan = new THREE.Vector3();
  eye.copy(self_scope.object.position).sub(self_scope.target);
  const objectUp = new THREE.Vector3();

  pan.copy(eye).cross(self_scope.object.up).setLength(-x / 100);
  pan.add(objectUp.copy(self_scope.object.up).setLength(y / 100));

  self_scope.object.position.add(pan)
  self_scope.target.add(pan);
}

const camera_speed=0.01;

function selfzoomcamera(self_scope,z){
  self_scope.object.zoom *= (1+camera_speed*z);
  self_scope.object.updateProjectionMatrix();
}

function resetcamera(self_scope){
  self_scope.reset();
  self_scope.update();
}

const Controls = ({ }) => {
  const controls = React.useRef();
  const { camera, gl } = useThree();
 
  const camera_vec = useSelector(state => state.camera)
  const dispatch = useDispatch();

  useFrame(() => {
    // update the view as the vis is interacted with
    // camera.x+=camera_vec.x/10
    // camera.y+=camera_vec.y/10
    // camera.z+=camera_vec.z/10
    if (camera_vec.x != 0 && camera_vec.y != 0) {
      pancamera(controls.current,camera_vec.x,camera_vec.y);
    }
    // if (camera_vec.zoom!=BASE){
    //   selfzoomcamera(controls.current,2);
    // }else{
      
    // }
    controls.current.update();
  });
  // const origin_zoom=controls.current.zoom;
  // const max_zoom=origin_zoom*();
  // const min_zoom=origin_zoom/BASE;
  useEffect(() => {
    if (camera_vec.r==true){
      resetcamera(controls.current)
      dispatch(reset_camera(false))
    }
  }, [camera_vec.r])

  return (
    <trackballControls
      ref={controls}
      args={[camera, gl.domElement]}
      dynamicDampingFactor={0.1}
      keys={[
        ALT_KEY, // orbit
        CTRL_KEY, // zoom
        CMD_KEY, // pan
      ]}
      mouseButtons={{
        LEFT: THREE.MOUSE.PAN, // make pan the default instead of rotate
        MIDDLE: THREE.MOUSE.ZOOM,
        RIGHT: THREE.MOUSE.ROTATE,
      }}
    />
  );
};

export default Controls;
