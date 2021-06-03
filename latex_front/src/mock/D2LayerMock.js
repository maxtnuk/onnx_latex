const mock_d2 =[
    {
        // group 0
        group: 0,
        layers: [
            {
                layer_num: 0,
                op_name: "hello",
                output_shape: [40,20]
            },
            {
                layer_num: 1,
                op_name: "cnn",
                output_shape: [40,30,11,20]
            },
            {
                layer_num: 2,
                op_name: "max_pool",
                output_shape: [40,30,100,100]
            },
        ]
    }
]
export default mock_d2;