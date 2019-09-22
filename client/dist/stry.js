(function() {
    const __exports = {};
    let wasm;

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }
    function __wbg_elem_binding0(arg0, arg1, arg2) {
        wasm.__wbg_function_table.get(29)(arg0, arg1, addHeapObject(arg2));
    }
    function __wbg_elem_binding1(arg0, arg1, arg2) {
        wasm.__wbg_function_table.get(57)(arg0, arg1, arg2);
    }
    function __wbg_elem_binding2(arg0, arg1, arg2) {
        wasm.__wbg_function_table.get(29)(arg0, arg1, addHeapObject(arg2));
    }
    function __wbg_elem_binding3(arg0, arg1, arg2, arg3) {
        wasm.__wbg_function_table.get(37)(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
    }
    /**
    */
    __exports.render = function() {
        wasm.render();
    };

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function init(module) {

    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__widl_f_document_Window = function(arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_body_Document = function(arg0) {
        const ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            getObject(arg0).setAttribute(getStringFromWasm(arg1, arg2), getStringFromWasm(arg3, arg4));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_error_1_ = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__widl_f_location_Window = function(arg0) {
        const ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_pathname_Location = function(arg0, arg1) {
        try {
            const ret = getObject(arg1).pathname;
            const ret0 = passStringToWasm(ret);
            const ret1 = WASM_VECTOR_LEN;
            getInt32Memory()[arg0 / 4 + 0] = ret0;
            getInt32Memory()[arg0 / 4 + 1] = ret1;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_hash_Location = function(arg0, arg1) {
        try {
            const ret = getObject(arg1).hash;
            const ret0 = passStringToWasm(ret);
            const ret1 = WASM_VECTOR_LEN;
            getInt32Memory()[arg0 / 4 + 0] = ret0;
            getInt32Memory()[arg0 / 4 + 1] = ret1;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_search_Location = function(arg0, arg1) {
        try {
            const ret = getObject(arg1).search;
            const ret0 = passStringToWasm(ret);
            const ret1 = WASM_VECTOR_LEN;
            getInt32Memory()[arg0 / 4 + 0] = ret0;
            getInt32Memory()[arg0 / 4 + 1] = ret1;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_add_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3) {
        try {
            getObject(arg0).addEventListener(getStringFromWasm(arg1, arg2), getObject(arg3));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_cb_forget = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__widl_f_warn_4_ = function(arg0, arg1, arg2, arg3) {
        console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__widl_f_error_4_ = function(arg0, arg1, arg2, arg3) {
        console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__widl_f_log_4_ = function(arg0, arg1, arg2, arg3) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__widl_f_debug_4_ = function(arg0, arg1, arg2, arg3) {
        console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__widl_f_info_4_ = function(arg0, arg1, arg2, arg3) {
        console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_resolve_bacd3bf49c19a0f8 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_0fe88013efbd2711 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__widl_instanceof_Element = function(arg0) {
        const ret = getObject(arg0) instanceof Element;
        return ret;
    };
    imports.wbg.__widl_f_node_type_Node = function(arg0) {
        const ret = getObject(arg0).nodeType;
        return ret;
    };
    imports.wbg.__widl_f_remove_attribute_Element = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).removeAttribute(getStringFromWasm(arg1, arg2));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_append_child_Node = function(arg0, arg1) {
        try {
            const ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_insert_before_Node = function(arg0, arg1, arg2) {
        try {
            const ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_cancel_animation_frame_Window = function(arg0, arg1) {
        try {
            getObject(arg0).cancelAnimationFrame(arg1);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_HTMLInputElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__widl_f_type_HTMLInputElement = function(arg0, arg1) {
        const ret = getObject(arg1).type;
        const ret0 = passStringToWasm(ret);
        const ret1 = WASM_VECTOR_LEN;
        getInt32Memory()[arg0 / 4 + 0] = ret0;
        getInt32Memory()[arg0 / 4 + 1] = ret1;
    };
    imports.wbg.__widl_instanceof_HTMLTextAreaElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLTextAreaElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLTextAreaElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLSelectElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLSelectElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLSelectElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLProgressElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLProgressElement;
        return ret;
    };
    imports.wbg.__widl_instanceof_HTMLOptionElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLOptionElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLOptionElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_f_set_value_HTMLProgressElement = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__widl_instanceof_HTMLButtonElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLButtonElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLDataElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLDataElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLDataElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLMeterElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLMeterElement;
        return ret;
    };
    imports.wbg.__widl_instanceof_HTMLLIElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLLIElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLMeterElement = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__widl_instanceof_HTMLOutputElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLOutputElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLOutputElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLParamElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLParamElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLParamElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_f_set_value_HTMLLIElement = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__widl_f_set_checked_HTMLInputElement = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__widl_instanceof_HTMLMenuItemElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLMenuItemElement;
        return ret;
    };
    imports.wbg.__widl_f_set_checked_HTMLMenuItemElement = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ret0 = passStringToWasm(ret);
        const ret1 = WASM_VECTOR_LEN;
        getInt32Memory()[arg0 / 4 + 0] = ret0;
        getInt32Memory()[arg0 / 4 + 1] = ret1;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        const v0 = getStringFromWasm(arg0, arg1).slice();
        wasm.__wbindgen_free(arg0, arg1 * 1);
        console.error(v0);
    };
    imports.wbg.__wbg_globalThis_36c1f2e85948e420 = function() {
        try {
            const ret = globalThis.globalThis;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_self_73c7a601ff857345 = function() {
        try {
            const ret = self.self;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_window_ca735e04cb2b0566 = function() {
        try {
            const ret = window.window;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_global_99312a595fd2e761 = function() {
        try {
            const ret = global.global;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_newnoargs_6ad69a50998c5acb = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_4499dca0c553c196 = function(arg0, arg1) {
        try {
            const ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__widl_instanceof_Window = function(arg0) {
        const ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_new_1719c88e1a2035ea = function(arg0, arg1) {
        const state0 = {a: arg0, b: arg1};
        const cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_elem_binding3(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        try {
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_call_fdde574e8abf6327 = function(arg0, arg1, arg2) {
        try {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_then_3d6c84182dd53850 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ret0 = passStringToWasm(ret);
        const ret1 = WASM_VECTOR_LEN;
        getInt32Memory()[arg0 / 4 + 0] = ret0;
        getInt32Memory()[arg0 / 4 + 1] = ret1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm(arg0, arg1));
    };
    imports.wbg.__widl_f_first_child_Node = function(arg0) {
        const ret = getObject(arg0).firstChild;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_request_animation_frame_Window = function(arg0, arg1) {
        try {
            const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            return ret;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_target_Event = function(arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_closest_Element = function(arg0, arg1, arg2) {
        try {
            const ret = getObject(arg0).closest(getStringFromWasm(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_tag_name_Element = function(arg0, arg1) {
        const ret = getObject(arg1).tagName;
        const ret0 = passStringToWasm(ret);
        const ret1 = WASM_VECTOR_LEN;
        getInt32Memory()[arg0 / 4 + 0] = ret0;
        getInt32Memory()[arg0 / 4 + 1] = ret1;
    };
    imports.wbg.__widl_f_get_attribute_Element = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).getAttribute(getStringFromWasm(arg2, arg3));
        const ptr0 = isLikeNone(ret) ? 0 : passStringToWasm(ret);
        const len0 = WASM_VECTOR_LEN;
        const ret0 = ptr0;
        const ret1 = len0;
        getInt32Memory()[arg0 / 4 + 0] = ret0;
        getInt32Memory()[arg0 / 4 + 1] = ret1;
    };
    imports.wbg.__widl_f_prevent_default_Event = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        const ret = JSON.parse(getStringFromWasm(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_history_Window = function(arg0) {
        try {
            const ret = getObject(arg0).history;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_push_state_with_url_History = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        try {
            getObject(arg0).pushState(getObject(arg1), getStringFromWasm(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm(arg4, arg5));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_PopStateEvent = function(arg0) {
        const ret = getObject(arg0) instanceof PopStateEvent;
        return ret;
    };
    imports.wbg.__widl_f_state_PopStateEvent = function(arg0) {
        const ret = getObject(arg0).state;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg0);
        if (typeof(obj) !== 'string') return 0;
        const ptr = passStringToWasm(obj);
        getUint32Memory()[arg1 / 4] = WASM_VECTOR_LEN;
        const ret = ptr;
        return ret;
    };
    imports.wbg.__widl_f_remove_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3) {
        try {
            getObject(arg0).removeEventListener(getStringFromWasm(arg1, arg2), getObject(arg3));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_element_ns_Document = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            const ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm(arg1, arg2), getStringFromWasm(arg3, arg4));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_element_Document = function(arg0, arg1, arg2) {
        try {
            const ret = getObject(arg0).createElement(getStringFromWasm(arg1, arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_text_node_Document = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).createTextNode(getStringFromWasm(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__widl_instanceof_Node = function(arg0) {
        const ret = getObject(arg0) instanceof Node;
        return ret;
    };
    imports.wbg.__widl_instanceof_HTMLElement = function(arg0) {
        const ret = getObject(arg0) instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__widl_f_focus_HTMLElement = function(arg0) {
        try {
            getObject(arg0).focus();
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_remove_child_Node = function(arg0, arg1) {
        try {
            const ret = getObject(arg0).removeChild(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_set_text_content_Node = function(arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm(arg1, arg2);
    };
    imports.wbg.__widl_f_replace_child_Node = function(arg0, arg1, arg2) {
        try {
            const ret = getObject(arg0).replaceChild(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_next_sibling_Node = function(arg0) {
        const ret = getObject(arg0).nextSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper195 = function(arg0, arg1, arg2) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_elem_binding0(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbg_function_table.get(30)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        const ret = real;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper664 = function(arg0, arg1, arg2) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_elem_binding1(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbg_function_table.get(30)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        const ret = real;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper165 = function(arg0, arg1, arg2) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_elem_binding2(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbg_function_table.get(30)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        const ret = real;
        return addHeapObject(ret);
    };

    if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                return response
                .then(r => {
                    if (r.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                        return r.arrayBuffer();
                    } else {
                        throw e;
                    }
                })
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();
