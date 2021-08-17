import { useCallback, useEffect, useRef, useState } from "react";
import React from "react";
import styled from "styled-components";
import { VictoryChart, VictoryScatter, VictoryTheme, VictoryAxis } from "victory";
import Button from "@material-ui/core/Button"
import TextField from '@material-ui/core/TextField';
import { useMemo } from "react";
import useWasm from "./usewasm";

import blob_csv from "./data/blob.csv"
import class_csv from "./data/classification.csv"
import gaussian_csv from "./data/gaussian.csv"
import data4_csv from "./data/data4.csv"



const MainPage = styled.div`
    width: 80%;
    margin: 0 auto;
`

const ChartContainer = styled.div`
    width: 40%;
    min-width: 50vh;
    position: relative;
`

const ControllContainer = styled.div`
    width: 100%;
    display: flex;
    flex-direction: row;
`

const VerticalContainer = styled.div`
    padding-top: 7rem;
    padding-bottom: 7rem;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
`
const VerticalControlContainer = styled.div`
    margin: 2em;
    display: flex;
    flex-direction: column;
    justify-content: center;
`

function csvToArray(str, delimiter = ",") {
    // slice from start of text to the first \n index
    // use split to create an array from string by delimiter
    const headers = str.slice(0, str.indexOf("\n")).split(delimiter);

    // slice from \n index + 1 to the end of the text
    // use split to create an array of each csv value row
    const rows = str.slice(str.indexOf("\n") + 1).split("\n");

    // Map the rows
    // split values from each row into an array
    // use headers.reduce to create an object
    // object properties derived from headers:values
    // the object passed as an element of the array
    const arr = rows.map(function (row) {
        const values = row.split(delimiter);
        const el = headers.reduce(function (object, header, index) {
            object[header] = values[index];
            return object;
        }, {});
        return el;
    });

    // return the array
    arr.pop();
    return arr;
}

function train_data({ data, map_ratio, raw_config,ctx,edge,margin,color_list }, wasm_instance) {
    const { train, ClusterConfig } = wasm_instance;
    const data_string = JSON.stringify(data.data);

    var config = ClusterConfig.new();

    config.classes = raw_config.classes;
    config.epochs = raw_config.epoch;
    config.learning_rate = raw_config.learning_rate;

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

// const createWorker = createWorkerFactory(() => import('./wasm.worker'));

function App() {
    const { error, loading, initialize, wasmInstance } = useWasm()
    const [sampledata, setsampledata] = useState(undefined)
    const [data, setdata] = useState(undefined)
    const [dindex, setdindex] = useState(-1)
    const [epoch, setepoch] = useState(10)
    const [learning_rate, setlearning_rate] = useState(0.001)
    const CanvasRef = useRef(null)
    const [time, settime] = useState(0)
    const wasmWorker = new Worker(new URL('./worker.js', import.meta.url));


    useEffect(() => {
        async function fetch_csv(csv_list) {
            var sample_list = []
            for (const ele of csv_list) {
                const name = ele["name"]
                const path = ele["path"]

                let e = await fetch(path).then(response => {
                    return response.text()
                }).then(data => {
                    return csvToArray(data)
                });

                const refined_data = e.map(v => {
                    return { X: parseFloat(v.X), Y: parseFloat(v.Y), Class: parseInt(v.Class) }
                })

                const classes = new Set(refined_data.map(v => v.Class)).size;
                sample_list.push({
                    name: name,
                    data: refined_data,
                    classes: classes
                })
            }
            setsampledata(sample_list)
        }
        async function init_wasm() {
            await initialize()
        }
        fetch_csv([
            {
                name: "blob",
                path: blob_csv
            },
            {
                name: "gaussian",
                path: gaussian_csv
            },
            {
                name: "data3",
                path: class_csv
            },
            {
                name: "data4",
                path: data4_csv
            }
        ]);
        init_wasm();
    }, []);

    let setSample = useCallback(
        i => {
            const blob_data = sampledata[i].data
            var datas = {};
            for (const ele of blob_data) {
                const class_name = ele.Class
                if (datas.hasOwnProperty(class_name)) {
                    datas[class_name].push({ x: ele.X, y: ele.Y })
                } else {
                    datas[class_name] = []
                }
            }
            setdata(datas)
        },
        [dindex],
    )

    useEffect(() => {
        if (sampledata != undefined) {
            setdindex(0)
        }
    }, [sampledata])

    const colors = ["#c43a31", "#31c487", "#3147c4", "#bae912"]
    const step_ratio = 50;
    const canvas_edge = 500;
    const margin = 50;


    // const hextorgb = (hex, alpha) => {

    //     const r = parseInt(hex.slice(1, 3), 16);
    //     const g = parseInt(hex.slice(3, 5), 16);
    //     const b = parseInt(hex.slice(5, 7), 16);

    //     if (alpha) {
    //         return `rgba(${r}, ${g}, ${b}, ${alpha})`;
    //     } else {
    //         return `rgb(${r}, ${g}, ${b})`;
    //     }
    // }

    const hextorgbraw = (hex) => {

        const r = parseInt(hex.slice(1, 3), 16);
        const g = parseInt(hex.slice(3, 5), 16);
        const b = parseInt(hex.slice(5, 7), 16);

        return {
            red: r,
            green: g,
            blue: b
        }
    }

    useEffect(() => {
        if (dindex != -1) {
            setSample(dindex);
            const canvas = CanvasRef.current;
            const ctx = canvas.getContext("2d");
            const height = canvas.height;
            const width = canvas.width;

            ctx.clearRect(0, 0, canvas.width, canvas.height);

        }
    }, [dindex])

    function async_train() {
        let data = sampledata[dindex];

        console.log(epoch)
        console.log(learning_rate)

        const raw_config = {
            epoch: epoch,
            learning_rate: learning_rate,
            classes: data.classes
        }
        // const on_error = (err) => {
        //     console.log(`error: ${err}`);
        // };
        // const on_epoch = (encoded) => {
        //     const data_array = JSON.parse(encoded);
        //     const canvas = CanvasRef.current;
        //     const ctx = canvas.getContext("2d");

        //     let rect = (canvas_edge - 2 * margin) / step_ratio;

        //     let base = [margin, margin]

        //     ctx.clearRect(0, 0, canvas.width, canvas.height);

        //     for (let i = 0; i < rects.length; ++i) {

        //         let [d, conf] = data_array[i];

        //         let x = rects[i][0];
        //         let y = rects[i][1];
        //         ctx.fillStyle = hextorgb(colors[d], conf);

        //         ctx.fillRect(base[0] + x * rect, base[1] + y * rect, rect, rect);
        //     }
        //     requestAnimationFrame()
        // };
        const map_ratio = step_ratio;
        const ctx = CanvasRef.current.getContext("2d");
        const edge = canvas_edge;
        const color_list = JSON.stringify(colors.map(e=>hextorgbraw(e)))
        const t0 = performance.now();
        train_data({ data, map_ratio, raw_config,ctx,edge,margin,color_list }, wasmInstance);
        const t1 = performance.now();
        settime(t1 - t0);
        // wasmWorker.postMessage({ data, map_ratio, raw_config,ctx,edge,margin,color_list })
    }

    return <>
        <MainPage>
            <h1>Data cluster simulation</h1>
            <ControllContainer>
                <ChartContainer>
                    {
                        data != undefined &&
                        <VictoryChart
                            width={canvas_edge}
                            height={canvas_edge}
                            domainPadding={10}
                            theme={VictoryTheme.material}
                            style={{
                                background: { fill: "#ffffff" },
                                position: "absolute"
                            }}
                        // backgroundComponent={<CustomBackground />}
                        >
                            <VictoryAxis

                                style={{
                                    grid: { stroke: "transparent" },
                                    axis: { stroke: "transparent" },
                                    ticks: { stroke: "transparent" },
                                    tickLabels: { fill: "transparent" }
                                }} />

                            {Object.entries(data).map(each_data => {
                                return <VictoryScatter
                                    key={`scatter_${each_data[0]}`}
                                    style={{ data: { fill: colors[parseInt(each_data[0])] } }}
                                    size={5}
                                    animate={{
                                        duration: 1000,
                                        onLoad: { duration: 500 }
                                    }}
                                    data={each_data[1]}
                                />
                            })
                            }
                        </VictoryChart>
                    }
                    <canvas
                        style={{
                            position: "absolute",
                            top: 0,
                            left: 0,
                            width: "100%"
                        }}
                        ref={CanvasRef}
                        height={canvas_edge}
                        width={canvas_edge}
                    >

                    </canvas>
                </ChartContainer>
                <VerticalContainer>
                    {sampledata != undefined && sampledata.map((x, i) => {
                        const name = x.name;
                        return <>
                            <Button
                                key={`sample_${i}`}
                                variant="contained"
                                color="primary"
                                onClick={() => {
                                    setdindex(i);
                                }}
                            >
                                {name}
                            </Button>
                        </>
                    })
                    }
                </VerticalContainer>
                <VerticalControlContainer>
                    <TextField
                        defaultValue={learning_rate}
                        style={
                            {
                                margin: "1rem"
                            }
                        }
                        label="Learning rate"
                        type="number"
                        onChange={(v) => {
                            if (v.target.value != "") {
                                setlearning_rate(parseFloat(v.target.value));
                            }
                        }}
                    />
                    <TextField
                        defaultValue={epoch}
                        style={
                            {
                                margin: "1rem"
                            }
                        }
                        label="Epoch"
                        type="number"
                        onChange={(v) => {
                            if (v.target.value != "") {
                                setepoch(parseInt(v.target.value));
                            }
                        }}
                    />
                    <Button
                        variant="contained"
                        color="secondary"
                        onClick={() => {
                            async_train()
                        }}
                    >
                        Train it!!
                    </Button>
                </VerticalControlContainer>
            </ControllContainer>
            <h1>train time: {(time/1000).toFixed(2)} second</h1>
        </MainPage>
    </>
}

export default App;