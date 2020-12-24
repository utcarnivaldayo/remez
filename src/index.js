
import init, { norm, substruct, wrap_remez } from '../wasm/wasm_test.js';

//remez
let order = 10;
let f_pass = 0.2;
let f_stop = 0.3;
let error = 0.0;
let buffer_response = new ArrayBuffer(8 * 501);
let magnitude_response = new Float64Array(buffer_response);
let buffer_axis = new ArrayBuffer(8 * 501);
let axis_frequency = new Float64Array(buffer_axis);
let buffer_coefficients = new ArrayBuffer(8 * (order * 2 + 1));
let coefficients = new Float64Array(buffer_coefficients);

function MagnitudeResponse(axis_data, mag_res) {
    //chart
    let ctx = document.getElementById('magResponse');
    let myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: axis_data,
            datasets: [
                {
                    label: "Magnitude response",
                    data: mag_res,
                    borderColor: "rgba(2550,0,0,1)",
                    backgroundColor: "rgba(0,0,0,0)",
                    pointRadius: 0
                }
            ],
        },
        options: {
            title: {
                display: true,
                position: 'bottom',
                text: 'Normalized frequency'
            },
            scales: {
                xAxes: [{
                    ticks: {
                        min: 0.0,
                        max: 0.5,
                        maxTicksLimit: 10,
                        precision: 0.05
                    }
                }],
                yAxes: [{
                    ticks: {
                        stepSize: 5,
                        callback: function (value, index, values) {
                            return value + "dB"
                        }
                    }
                }]
            },
            //maintainAspectRatio: false
        }
    }
    )
}

async function run() {

    await init();
    Update();
}

function Update() {
    //get data
    const ord = document.getElementById("order");
    const passband = document.getElementById("passband");
    const stopband = document.getElementById("stopband");

    order = ord.value;
    f_pass = passband.value;
    f_stop = stopband.value;

    //remez
    error = wrap_remez(order, f_pass, f_stop, coefficients, axis_frequency, magnitude_response);

    //chart
    MagnitudeResponse(axis_frequency, magnitude_response);

    document.getElementById("error").innerText = "設計誤差 : ";
    document.getElementById("error").insertAdjacentText('beforeend', error);

    document.getElementById("coef").innerHTML = "フィルタ係数 :<br>";
    for (let i = 0; i < coefficients.length; i++) {
        document.getElementById("coef").insertAdjacentHTML('beforeend', "h_" + i + " : " + coefficients[i] + "<br>");
    }

}

document.getElementById('btnrun').onclick = function () {
    Update();
}

run();