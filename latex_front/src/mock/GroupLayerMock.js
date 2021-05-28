const mock_groups =[
    {
        // group 0
        group: 0,
        layers: [
            {
                op_type: "cnn",
                layer_num: 0,
                channel: 25,
                width: 100,
                height: 100
            },
            {
                op_type: "cnn",
                layer_num: 1,
                channel: 25,
                width: 100,
                height: 100
            },
            {
                op_type: "sum_pool",
                layer_num: 2,
                channel: 25,
                width: 15,
                height: 15
            },
            {
                op_type: "max_pool",
                layer_num: 3,
                channel: 25,
                width: 32,
                height: 32
            },
            {
                op_type: "cnn",
                layer_num: 4,
                channel: 20,
                width: 25,
                height: 25
            },
            {
                op_type: "max_pool",
                layer_num: 5,
                channel: 30,
                width: 25,
                height: 25
            },
        ]
    },
    {
        // group 1
        group: 1,
        layers: [
            {
                op_type: "cnn",
                layer_num: 0,
                channel: 25,
                width: 120,
                height: 120
            },
            {
                op_type: "max_pool",
                layer_num: 1,
                channel: 25,
                width: 30,
                height: 30
            },
            {
                op_type: "max_pool",
                layer_num: 2,
                channel: 10,
                width: 25,
                height: 2
            },
        ]
    },
    {
        // group 2
        group: 2,
        layers: [
            {
                op_type: "cnn",
                layer_num: 0,
                channel: 25,
                width: 25,
                height: 25
            }
        ]
    },
    {
        // group 3
        group: 3,
        layers: [
            {
                op_type: "sum_pool",
                layer_num: 0,
                channel: 100,
                width: 100,
                height: 100
            },
            {
                op_type: "cnn",
                layer_num: 1,
                channel: 64,
                width: 64,
                height: 64
            },
            {
                op_type: "cnn",
                layer_num: 2,
                channel: 32,
                width: 32,
                height: 32
            },
            {
                op_type: "cnn",
                layer_num: 3,
                channel: 25,
                width: 16,
                height: 16
            },
            {
                op_type: "max_pool",
                layer_num: 4,
                channel: 1,
                width: 25,
                height: 1
            },
        ]
    },
    {
        // group 4
        group: 4,
        layers: [
            {
                op_type: "cnn",
                layer_num: 0,
                channel: 10,
                width: 10,
                height: 10
            },
            {
                op_type: "sum_pool",
                layer_num: 1,
                channel: 12,
                width: 12,
                height: 12
            },
            {
                op_type: "cnn",
                layer_num: 2,
                channel: 70,
                width: 10,
                height: 5
            },
            {
                op_type: "cnn",
                layer_num: 3,
                channel: 2,
                width: 25,
                height: 2
            },
        ]
    },
]
export default mock_groups