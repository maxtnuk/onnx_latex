import { useState } from "react"
import { useCallback } from "react"
import { useEffect } from "react";
import { debounce } from "debounce";
import { URLSearchParams } from "url";
import axios from "axios";

const client=axios.create({
    baseURL: "http://127.0.0.1:1234",
    
});

export function useGetModel(model_request) {
    const [error, seterror] = useState({})
    const [senario, setsenario] = useState([])
    const [symbol_map, setsymbol_map] = useState([])
    const [during, setduring] = useState(false)

    const delayed_fetch= useCallback(
        debounce((request_content) => {
            console.log("debounce")
            send_model(request_content)
        },500),
        [],
    )

    const send_model = useCallback(
        async (request_content) => {
            // this mean initial request
            console.log(request_content)
            if (request_content.depth===-1){
                return
            }
            try {
                setduring(true);
                let formdata = new FormData();
                formdata.append('model',request_content.file)
                console.log(formdata)
                let res = await client.post(
                    "/parse_model", formdata, {
                    params: {
                        depth: request_content.depth
                    }
                }
                )
                console.log(res)
                if (res.status == 200) {
                    const data = res.data
                    setsenario(data.senario);
                    setsymbol_map(data.symbol_map);
                }
                setduring(false);
            } catch (error) {
                console.log(error)
                seterror(error);
                setduring(false);
            }
        }
        , [model_request])
    
    useEffect(() => {
        delayed_fetch(model_request);
    }, [model_request])
    return { error, during, senario, symbol_map }
}