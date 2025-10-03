export function cent(price: number): string {
    return price % 100 == 0 ? " " : String(price % 100);
}

export function euro(price: number): string {
    return String(Math.floor(price / 100));
}

export function money(price: number): string {
    return String(price / 10);
}