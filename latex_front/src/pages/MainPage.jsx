import React from "react";
import LayerImage from "components/LayerImage";
import { MathJaxContext } from "better-react-mathjax";
import { MathJax } from "better-react-mathjax";

const div_style={
  height: "100px",
}
const inner_style={
  
}


function MainPage() { 
  return (
    <>
    <div style={div_style}>
      <LayerImage 
        style={inner_style}
        x={2} y={3} z={3}
        width={0.1}
        stroke="black"
        color="yellow"
        padding={3}
        many={10}
        ></LayerImage>
    </div>
    <div>
      <MathJaxContext>
              <h2>Basic MathJax example with Latex</h2>
              <MathJax>{"\\(cnn_1\\)"}</MathJax>
        </MathJaxContext>
    </div>
    </>
  );
}
export default MainPage;
