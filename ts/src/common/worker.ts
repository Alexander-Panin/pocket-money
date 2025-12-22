const map: Record<number, Function> = {};

function worker(): Worker { return (globalThis as any).__worker; }

export function onmsg(e: any) {
	const {id, msg} = e.data;
	if (map[id]) {
		map[id](msg);
		delete map[id];
	}
};

const send = (type: string, msg: object): Promise<object> =>
	new Promise((resolve) => {
		const id = Math.random();
		const payload = {id, type, msg};
		map[id] = resolve;
		worker().postMessage(payload);
	});

export default send;
