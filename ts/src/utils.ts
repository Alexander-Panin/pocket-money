export function cent(price: number): string {
    let x = Math.floor(price);
    return price == x ? " " : String(Math.round((price - x) * 100));
}

export function euro(price: number): string {
    return String(Math.floor(price)) + ",";
}

export function money(price: number, k: number = 1): string {
    return String(Math.round(price * 10) / 10);
}