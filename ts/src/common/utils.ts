export function getMonths(locale: string = 'en'): string[] {
  return locale === 'ru'
    ? ['январь', 'февраль', 'март', 'апрель', 'май', 'июнь', 'июль', 'август', 'сентябрь', 'октябрь', 'ноябрь', 'декабрь']
    : monthsRaw();
}

export function getMonthBy(month: string, locale: string): string {
  return getMonths(locale)[getMonths().indexOf(month)] ?? "not found"; 
}

export function monthsRaw() {
  return ['january', 'february', 'march', 'april', 'may', 'june', 'july', 'august', 'september', 'october', 'november', 'december'];
}

export function yearsRaw() {
  return ['2025', '2026'];
}

export function months2025() {
  return monthsRaw().slice(10); // just december and november
}

export function months2026() {
  return monthsRaw().slice(0,5); 
}

export function calendarKeys() {
  const result = [];
  for (const month of months2025())
      result.push(`2025:${month}`);
  for (const month of months2026())
      result.push(`2026:${month}`);
  return result;
}