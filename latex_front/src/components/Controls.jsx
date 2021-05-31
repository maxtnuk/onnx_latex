import * as React from 'react';
import { extend, useThree, useFrame } from '@react-three/fiber';
import { TrackballControls } from 'three/examples/jsm/controls/TrackballControls';
import * as THREE from 'three';
import { useSelector, useDispatch } from 'react-redux'
import { useEffect, useState,useRef } from 'react';
import { is_dragging, zoom_camera } from 'api/camera';
import { BASE } from './ZoomBar';
import {reset_camera } from 'api/camera'
import { OrbitControls } from '@react-three/drei';

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
function flat_horizon(self_scope){
  
  self_scope.update();
}

const Controls = ({ }) => {
  const controls = useRef();
  const { camera, gl } = useThree();
 
  const camera_vec = useSelector(state => state.camera)
  const dispatch = useDispatch();

  useFrame(() => {
    if (camera_vec.x != 0 && camera_vec.y != 0) {
      pancamera(controls.current,camera_vec.x,camera_vec.y);
    }
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

  useEffect(() => {
    // if (controls.current){
    //   controls.current.addEventListener("start", (event)=>{
    //     dispatch(is_dragging(true))
    //   })
    //   controls.current.addEventListener("end",(event) => {
    //     setTimeout(()=> {
    //       dispatch(is_dragging(false))
    //     },10000,"end drag")
    //   })
    // }
  })

  return (<>
    <OrbitControls
      ref={controls}
      camera={camera}
      dampingFactor={0.05}
    />
    </>
  );
};

export default Controls;
