const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const path = require('path');
const fs = require('fs')
const os = require('os');

const jschardet = require('jschardet');

var mainWin = null;//mainwindow
var resultWin = null;//resultwindow

var filePath = null;

var jsonObject = null;
var processedData = null;

ipcMain.on('file-choose', function (event) {//when file choose button pressed
    if (os.platform() === 'linux' || os.platform() === 'win32') {
        filePath = dialog.showOpenDialogSync({
            properties: ['openFile']//file can be chose
        });
        console.log(filePath[0]);
        //현재는 그냥 콘솔창에 파일 위치 출력 나중에 용연이한테 경로 넘길 파일위치 저장 
        //여기서 용연이를 통해서 파일을 로컬서버로 전송
    } else {//mac OS
        filePath = dialog.showOpenDialogSync({
            properties: ['openFile', 'openDirectory']//file and directory can be chose
        });
        console.log(filePath[0]);//여기도 마찬가지
    }


    //대충 로컬서버에서 파일을 가져왔다고 둠 이 부분은 나중에 이벤트로 해야함 일단 그걸 현재 구현 불가능하니 이 이벤트에서 실행을 함
    //코드 자체를 스트링화 시켜 그 다음에 nodejs에서 json library 이용해서 parsing을 진행해 그런 다음에 돌리면 될듯

    processedData = `{
        “symbol_map”: [
          {
            “inputs”: [],
            “symbol”: “\\overline{Input}“,
            “value”: “\\overline{Input}“,
            “shape”: [
              64,
              1000
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [],
            “symbol”: “\\overline{B_2}“,
            “value”: “\\overline{B_2}“,
            “shape”: [
              100
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [],
            “symbol”: “\\overline{B_3}“,
            “value”: “\\overline{B_3}“,
            “shape”: [
              10
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [],
            “symbol”: “\\overline{B_1}“,
            “value”: “\\overline{B_1}“,
            “shape”: [
              100
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [],
            “symbol”: “\\overline{W_3}“,
            “value”: “\\overline{W_3}“,
            “shape”: [
              10,
              100
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [],
            “symbol”: “\\overline{W_1}“,
            “value”: “\\overline{W_1}“,
            “shape”: [
              100,
              1000
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [],
            “symbol”: “\\overline{W_2}“,
            “value”: “\\overline{W_2}“,
            “shape”: [
              100,
              100
            ],
            “forward_prefix”: “”,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [
              0,
              5,
              3
            ],
            “symbol”: “f_0”,
            “value”: “{\\overline{Input}}\\cdot {\\overline{W_1}}+{\\overline{B_1}}“,
            “shape”: [
              64,
              100
            ],
            “forward_prefix”: “{#_0}\\cdot {#_1}+{#_2}“,
            “backward_prefix”: “#”,
            “backward_value”: “\\sum_{n_0=0}^{99} (\\sum_{n_1=0}^{9} (\\frac{\\partial E_{(total,{h_2}_{n_1})}}{\\partial {h_2}_{n_1}} \\times \\frac{\\partial {h_2}_{n_1}}{\\partial {f_2}_{n_1}} \\times \\frac{\\partial {f_2}_{n_1}}{\\partial {h_1}_{n_0}}) \\times \\frac{\\partial {h_1}_{n_0}}{\\partial {f_1}_{n_0}} \\times \\frac{\\partial {f_1}_{n_0}}{\\partial {h_0}_{(9)}}) \\times \\frac{\\partial {h_0}_{(9)}}{\\partial {f_0}_{(9)}} \\times \\frac{\\partial {f_0}_{(9)}}{\\partial {w}_{({f_0}_{(9)},4)}}“,
            “backward_symbol”: “\\frac{\\partial E_{(total,)}}{\\partial {w}_{({f_0}_{(9)},4)}}“,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [
              7
            ],
            “symbol”: “h_0",
            “value”: “min(max({\\overline{Input}}\\cdot {\\overline{W_1}}+{\\overline{B_1}},0),None)“,
            “shape”: [
              64,
              100
            ],
            “forward_prefix”: “min(max(#_0,@_0),@_1)“,
            “backward_prefix”: “”,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Tuple”: [
                {
                  “Tuple”: [
                    {
                      “Num”: 0.0
                    }
                  ]
                },
                {
                  “Undefined”: “None”
                }
              ]
            }
          },
          {
            “inputs”: [
              8,
              6,
              1
            ],
            “symbol”: “f_1”,
            “value”: “{min(max({\\overline{Input}}\\cdot {\\overline{W_1}}+{\\overline{B_1}},0),None)}\\cdot {\\overline{W_2}}+{\\overline{B_2}}“,
            “shape”: [
              64,
              100
            ],
            “forward_prefix”: “{#_0}\\cdot {#_1}+{#_2}“,
            “backward_prefix”: “#”,
            “backward_value”: “\\sum_{n_0=0}^{9} (\\frac{\\partial E_{(total,{h_2}_{n_0})}}{\\partial {h_2}_{n_0}} \\times \\frac{\\partial {h_2}_{n_0}}{\\partial {f_2}_{n_0}} \\times \\frac{\\partial {f_2}_{n_0}}{\\partial {h_1}_{(9)}}) \\times \\frac{\\partial {h_1}_{(9)}}{\\partial {f_1}_{(9)}} \\times \\frac{\\partial {f_1}_{(9)}}{\\partial {w}_{({f_1}_{(9)},4)}}“,
            “backward_symbol”: “\\frac{\\partial E_{(total,)}}{\\partial {w}_{({f_1}_{(9)},4)}}“,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [
              9
            ],
            “symbol”: “h_1",
            “value”: “\\frac{1}{1+e^{-({min(max({\\overline{Input}}\\cdot {\\overline{W_1}}+{\\overline{B_1}},0),None)}\\cdot {\\overline{W_2}}+{\\overline{B_2}})}}“,
            “shape”: [
              64,
              100
            ],
            “forward_prefix”: “\\frac{1}{1+e^{-(#_0)}}“,
            “backward_prefix”: “$_0(1-$_0)“,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Tuple”: [
                {
                  “Undefined”: “Sigmoid”
                }
              ]
            }
          },
          {
            “inputs”: [
              10,
              4,
              2
            ],
            “symbol”: “f_2”,
            “value”: “{\\frac{1}{1+e^{-({min(max({\\overline{Input}}\\cdot {\\overline{W_1}}+{\\overline{B_1}},0),None)}\\cdot {\\overline{W_2}}+{\\overline{B_2}})}}}\\cdot {\\overline{W_3}}+{\\overline{B_3}}“,
            “shape”: [
              64,
              10
            ],
            “forward_prefix”: “{#_0}\\cdot {#_1}+{#_2}“,
            “backward_prefix”: “#”,
            “backward_value”: “\\frac{\\partial E_{(total,{h_2}_{(9)})}}{\\partial {h_2}_{(9)}} \\times \\frac{\\partial {h_2}_{(9)}}{\\partial {f_2}_{(9)}} \\times \\frac{\\partial {f_2}_{(9)}}{\\partial {w}_{({f_2}_{(9)},4)}}“,
            “backward_symbol”: “\\frac{\\partial E_{(total,)}}{\\partial {w}_{({f_2}_{(9)},4)}}“,
            “op_attributes”: {
              “Undefined”: “”
            }
          },
          {
            “inputs”: [
              11
            ],
            “symbol”: “h_2",
            “value”: “\\frac{1}{1+e^{-({\\frac{1}{1+e^{-({min(max({\\overline{Input}}\\cdot {\\overline{W_1}}+{\\overline{B_1}},0),None)}\\cdot {\\overline{W_2}}+{\\overline{B_2}})}}}\\cdot {\\overline{W_3}}+{\\overline{B_3}})}}“,
            “shape”: [
              64,
              10
            ],
            “forward_prefix”: “\\frac{1}{1+e^{-(#_0)}}“,
            “backward_prefix”: “$_0(1-$_0)“,
            “backward_value”: “”,
            “backward_symbol”: “”,
            “op_attributes”: {
              “Tuple”: [
                {
                  “Undefined”: “Sigmoid”
                }
              ]
            }
          }
        ],
        “senario”: [
          7,
          8,
          9,
          10,
          11,
          12
        ]
      }`;

    processedData = processedData.replace(/“/g, '"');//unicode change “ and  ”
    processedData = processedData.replace(/”/g, '"');
    processedData = processedData.replace(/\\/g, '\\\\');//reading \\ changes \, so reading \\\\ changes \\. it doesn't make error json.parse

    resultWin = new BrowserWindow({//make resultWindow
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        }
    });

    resultWin.loadFile('result.html');

    jsonObject = JSON.parse(processedData);//string to json 

    resultWin.webContents.on('did-finish-load', () => {//when loading end
        resultWin.webContents.send('result-channel', jsonObject);//send data
    });
});


app.whenReady().then(() => {
    mainWin = new BrowserWindow({
        resizable: true,
        height: 600,
        width: 800,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        }
    });

    mainWin.loadFile('index.html');
});


app.on('window-all-closed', () => {//when all window closed
    if (process.platform !== 'darwin') {
        app.quit();
    }
});

