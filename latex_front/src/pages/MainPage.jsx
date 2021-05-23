import * as THREE from 'three'
// import { SVGRenderer } from '../resource/SVGRenderer'
import React from "react";
import LayerImage from "components/LayerImage";
import { MathJaxContext } from "better-react-mathjax";
import { MathJax } from "better-react-mathjax";
import { Canvas } from '@react-three/fiber'
import styled from "styled-components";
import Controls from 'components/Controls';

const VisContainer = styled.div`
  height: 100vh;
  width: 100vw;
`;

// const renderer = new SVGRenderer()
// renderer.setSize(window.innerWidth, window.innerHeight)
// document.body.appendChild(renderer.domElement)
// const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000)
// const scene = new THREE.Scene()

function MainPage() { 
  return (
    <>
    <VisContainer>
      <Canvas camera={{ position: [0, 0, 10] }}>
        <Controls/>
        <ambientLight />
        <pointLight position={[10, 10, 10]} />
        <LayerImage position={[-1.2, 0, 0]} />
        <LayerImage position={[1.2, 0, 0]} />
      </Canvas>
    </VisContainer>
    
    {/* <div>
      <MathJaxContext>
              <h2>Basic MathJax example with Latex</h2>
              <MathJax>{"\\(cnn_1\\)"}</MathJax>
        </MathJaxContext>
    </div> */}
    </>
  );
}
export default MainPage;
