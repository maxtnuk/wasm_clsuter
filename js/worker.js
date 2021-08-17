function train_data({ data, map_ratio, raw_config,ctx,edge,margin,color_list }, wasm_instance) {
    const { train, ClusterConfig } = wasm_instance;
    const data_string = JSON.stringify(data.data);

    var config = ClusterConfig.new();

    config.classes = raw_config.classes;
    config.epochs = raw_config.epoch;
    config.learning_rate = raw_config.learning_rate;

    console.log(edge)
    console.log(margin)
    console.log(color_list)
    train(
        data_string,
        map_ratio,
        config,
        ctx,
        edge,
        margin,
        color_list
    )
}


self.onmessage=async ({data})=>{
    const { data, map_ratio, raw_config,ctx,edge,margin,color_list } = data;
    const wasm_instance=await import("../pkg/index.js");
    train_data({ data, map_ratio, raw_config,ctx,edge,margin,color_list },wasm_instance)
}