type Day = {
    id: string;
    date: number;
    price: number;
    tag: string;
    comment: string;
    save: () => void;
};

type Wasm = Record<string, any>;
