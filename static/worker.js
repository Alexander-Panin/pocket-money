import init, * as wasm from "/script/pkg/wasm.js?t=1768852079226";

self.onmessage = async (e) => {
    await init();
    const { id: mid, type, msg } = e.data;
    const {id, value, ns, prevNs} = msg;
    const sendBack = x => self.postMessage({id: mid, msg: x});
    switch (type) {
        case "append": 
            wasm.Store.append(ns, id).then(sendBack); 
            break;
        case "index:write": 
            wasm.Index.write(ns, id, value).then(sendBack); 
            break;
        case "repeat_regular": 
            wasm.Store.repeat_regular(ns, prevNs).then(sendBack); 
            break;
        case "save_price": 
            wasm.save_price(id, value).then(sendBack); 
            break;
        case "save_tag": 
            wasm.save_tag(id, value).then(sendBack); 
            break;
        case "save_date": 
            wasm.save_date(id, value).then(sendBack); 
            break;
        case "save_comment": 
            wasm.save_comment(id, value).then(sendBack); 
            break;
    }
}
