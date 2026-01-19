export function getMonths(locale: string = 'en'): string[] {
  return locale === 'ru'
    ? ['январь', 'февраль', 'март', 'апрель', 'май', 'июнь', 'июль', 'август', 'сентябрь', 'октябрь', 'ноябрь', 'декабрь']
    : ['january', 'february', 'march', 'april', 'may', 'june', 'july', 'august', 'september', 'october', 'november', 'december'];
}

export function getMonthBy(month: string, locale: string): string {
  return getMonths(locale)[getMonths().indexOf(month)] ?? "not found"; 
}

