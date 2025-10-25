import { getMonths } from "../common/utils";

export function getNamespace(search: string): string {
    const params = new URLSearchParams(search);
    const [a,m] = [parseInt(params.get('year') ?? "0"), params.get('month')];
    const year = a > 1970 && a < 2125 ? a : new Date().getFullYear();
    const month = getMonths().find(x => x === m) ?? getMonths()[new Date().getMonth()];
    return `${year}:${month}`;
}

type Params = { year: number, month: string };
export function getParams(ns: string): Params {
  const [year = '', month = ''] = ns.split(":");
  return {
    year: parseInt(year) || 0,
    month,
  };
} 

