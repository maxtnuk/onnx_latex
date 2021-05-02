const { ipcRenderer } = require('electron');

var processedData = null;

var t = document.getElementById('example');
t.addEventListener('click', (event) => {
    alert('hello' + event.target.value);
});

ipcRenderer.on('result-channel', (event, data) => {//데이터를 받은 후에야 시작한다. 이 부분은 나중에 좀 더 깔끔하게 변경 가능할듯
    processedData = data;


    for (var i = 0; i < processedData.senario.length; i++) {

        var j = processedData.senario[i];//실행되는 symbol_map의 레이어 순서
        var target = document.getElementById('example');//붙일 곳
        var li = document.createElement('ul');//붙일 것 생성
        li.innerHTML = "$$" + processedData.symbol_map[j].value + "$$";
        target.appendChild(li);//붙이기 
    }
});