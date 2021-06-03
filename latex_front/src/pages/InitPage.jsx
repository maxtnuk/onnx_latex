import React, { useCallback, useState, useRef } from "react";
import styled from "styled-components";
import Dropzone from "react-dropzone";
import { useDispatch } from "react-redux";
import { useGetModel } from "api/rest_api";
import { useEffect } from "react";
import { Modal } from "@material-ui/core";
import ReactInterval from "react-interval";
import LoadingModel from "components/LoadingModel";
import { forwardRef } from "react";

const RootDiv = styled.div`
  overflow: hidden;
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  display: flex;
  align-items: center;
  justify-content:center;
`;

const CenterContent = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;
`;

const Title = styled.div`
  font-size: 60px;
`;

const DropZoneDiv = styled.div`
  text-align: center;
  padding: 20px;
  border: 3px dashed #a50d0d;
  background-color: #5c921e;
  color: #fdfdff;

  margin-bottom: 20px;
`;

const ForwardSpin = forwardRef((props,ref)=>{
  return (
    <LoadingModel
    {...props}
    ref={ref}
  >

  </LoadingModel>
  )
})


function InitPage() {

  const dispatch = useDispatch(state => state.model)
  const depth = 3;

  const [timerstart, settimerstart] = useState(false);
  const [loadingpopup, setloadingpopup] = useState(false);
  const [modelRequest, setmodelRequest] = useState({
    file: {},
    depth: -1
  })

  const { error, during, res_model } = useGetModel(modelRequest);

  const spin_ref = useRef();
  const timer = useRef();

  useEffect(() => {

  }, [])
  useEffect(() => {
    if (loadingpopup === true) {
      // close popup
      setloadingpopup(false);
    }
    settimerstart(false);
    console.log(res_model)
  }, [res_model])
  useEffect(() => {
    if (loadingpopup) {
      // close popup
      setloadingpopup(false);
    }
    settimerstart(false);
    console.log(error)

  }, [error])

  return (
    <RootDiv>
      {/* timer for loading */}
      <ReactInterval
        ref={timer}
        enabled={timerstart}
        timeout={2000}
        callback={() => { }}
      />
      <CenterContent>
        <Title>
          ML Expression
        </Title>
        <div style={{
          font_size: "11px",
        }}>
          please upload onnx file
        </div>
        <DropZoneDiv>
          <Dropzone
            onDrop={(files) => {
              // if it takes long, popup modal
              timer.current.callback = () => {
                console.log("time out");
                if (during) {
                  setloadingpopup(true);
                }
                settimerstart(false);
              };
              settimerstart(true);
              setmodelRequest({
                file: files[0],
                depth: depth
              })
            }}
            apply={"*.onnx"}
            maxFiles={1}
          >
            {({
              getRootProps,
              getInputProps,
              isDragActive,
              isDragAccept,
              isDragReject
            }) => {
              const additionalClass = isDragAccept
                ? "accept"
                : isDragReject
                  ? "reject"
                  : "";

              return (
                <div
                  {...getRootProps({
                    className: `dropzone ${additionalClass}`
                  })}
                >
                  <input {...getInputProps()} />
                  <span>{isDragActive ? "📂" : "📁"}</span>
                  <p>Drag'n'drop images, or click to select files</p>
                </div>
              );
            }}
          </Dropzone>
        </DropZoneDiv>
      </CenterContent>
      <Modal
        open={loadingpopup}
        aria-labelledby="simple-modal-title"
        aria-describedby="simple-modal-description"
      >
        <ForwardSpin
        ref={spin_ref}>
        </ForwardSpin>
      </Modal>
    </RootDiv>
  );
}

export default InitPage;
