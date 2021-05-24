import * as React from 'react';
import { extend, useThree, useFrame } from '@react-three/fiber';
import { TrackballControls } from 'three/examples/jsm/controls/TrackballControls';
import * as THREE from 'three';
import {useSelector } from 'react-redux'
import {useEffect, useState } from 'react';
import {MapControls} from '@react-three/drei'

// extend THREE to include TrackballControls
extend({ TrackballControls });

// key code constants
const ALT_KEY = 18;
const CTRL_KEY = 17;
const CMD_KEY = 91;

const Controls = ({}) => {
  const controls = React.useRef();
  const { camera, gl } = useThree();

  const camera_vec = useSelector(state => state.camera)

 
  useFrame(() => {
    // update the view as the vis is interacted with
    // camera.x+=camera_vec.x/10
    // camera.y+=camera_vec.y/10
    // camera.z+=camera_vec.z/10
    if (camera_vec.x!=0 && camera_vec.y!=0){
      const self_scope=controls.current
      const eye = new THREE.Vector3();
      eye.copy(self_scope.object.position).sub( self_scope.target );
      const pan = new THREE.Vector3();
      const objectUp = new THREE.Vector3();

      pan.copy( eye ).cross( self_scope.object.up ).setLength( -camera_vec.x/100);
      pan.add( objectUp.copy( self_scope.object.up ).setLength( camera_vec.y/100 ) );

      controls.current.object.position.add(pan)
      controls.current.target.add( pan );
    }
    controls.current.update();
  });
  
  

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
