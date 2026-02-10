import getWasm from "../common/wasm";

export function prefixHash(index: string, key: string) {
	const [xs, n] = str2ab(index);
	const [ys, k] = str2ab(key);
	let prefix = getWasm().PrefixHash.new(n);
	prefix.build(xs);
	const result = prefix.find(ys);
	result.forEach((i: number) => console.log(ab2str(xs.slice(i, i+k))));
}

function ab2str(buf: Uint16Array): String {
    return String.fromCharCode.apply(null, buf as unknown as number[]);
}

function str2ab(str: string): [Uint16Array, number] {
    const n = str.length;
    const buf = new ArrayBuffer(n*2); // two bytes for each char
    const bufView = new Uint16Array(buf);
    for (let i=0; i !== n; i++) {
    	bufView[i] = str.charCodeAt(i);
    }
    return [bufView, n];
}