import React, { useCallback, useState } from "react";
import styled from "styled-components";
import Dropzone from "react-dropzone";
import { useDispatch } from "react-redux";
import { useGetModel } from "api/rest_api";
import { useEffect } from "react";

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

function InitPage() {
  const [isLoading, setisLoading] = useState(false)

  const dispatch = useDispatch(state => state.model)
  const depth= 3;
  const [modelRequest, setmodelRequest] = useState({
    file: {},
    depth: -1
  })

  const { error, during, senario, symbol_map } = useGetModel(modelRequest);
  useEffect(() => {
    console.log("symbol");
    console.log(symbol_map);
  }, [symbol_map])
  useEffect(() => {
   console.log("error");
   console.log(error);
  }, [error])
 
  return (
    <RootDiv>
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
              console.log("called")
              setmodelRequest({
                file: files[0],
                depth: depth
              })
            }}
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
    </RootDiv>
  );
}

export default InitPage;
