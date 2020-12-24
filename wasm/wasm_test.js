
let wasm;

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8);
    getFloat64Memory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {Float64Array} a
* @param {Float64Array} b
* @returns {number}
*/
export function norm(a, b) {
    var ptr0 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    var ret = wasm.norm(ptr0, len0, ptr1, len1);
    return ret;
}

/**
* @param {Float64Array} a
* @param {Float64Array} b
*/
export function substruct(a, b) {
    try {
        var ptr0 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        wasm.substruct(ptr0, len0, ptr1, len1);
    } finally {
        a.set(getFloat64Memory0().subarray(ptr0 / 8, ptr0 / 8 + len0));
        wasm.__wbindgen_free(ptr0, len0 * 8);
        b.set(getFloat64Memory0().subarray(ptr1 / 8, ptr1 / 8 + len1));
        wasm.__wbindgen_free(ptr1, len1 * 8);
    }
}

/**
* @param {number} order
* @param {number} f_pass
* @param {number} f_stop
* @param {Float64Array} coefficients
* @param {Float64Array} axis_freq
* @param {Float64Array} magnitude_response
* @returns {number}
*/
export function wrap_remez(order, f_pass, f_stop, coefficients, axis_freq, magnitude_response) {
    try {
        var ptr0 = passArrayF64ToWasm0(coefficients, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArrayF64ToWasm0(axis_freq, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = passArrayF64ToWasm0(magnitude_response, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ret = wasm.wrap_remez(order, f_pass, f_stop, ptr0, len0, ptr1, len1, ptr2, len2);
        return ret;
    } finally {
        coefficients.set(getFloat64Memory0().subarray(ptr0 / 8, ptr0 / 8 + len0));
        wasm.__wbindgen_free(ptr0, len0 * 8);
        axis_freq.set(getFloat64Memory0().subarray(ptr1 / 8, ptr1 / 8 + len1));
        wasm.__wbindgen_free(ptr1, len1 * 8);
        magnitude_response.set(getFloat64Memory0().subarray(ptr2 / 8, ptr2 / 8 + len2));
        wasm.__wbindgen_free(ptr2, len2 * 8);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};


    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

