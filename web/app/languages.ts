import { LANGUAGES, translateText, type Language } from './translations.ts';

const KEY = 'firecal-language';
const ATTRIBUTES = ['aria-label', 'title', 'placeholder', 'alt'] as const;
interface Source { english: string; rendered: string }

/** Keeps the English source of mutable text so changing language never translates a translation. */
export function installLanguages(root: HTMLElement): void {
  const select = root.querySelector<HTMLSelectElement>('#language-select');
  if (!select) return;
  let language: Language = 'en';
  try {
    const saved = localStorage.getItem(KEY);
    if (LANGUAGES.includes(saved as Language)) language = saved as Language;
  } catch { /* The default language works when browser storage is unavailable. */ }
  const texts = new WeakMap<Text, Source>();
  const attributes = new WeakMap<Element, Map<string, Source>>();
  const axes = new WeakMap<Element, string>();
  const observer = new MutationObserver(() => { if (!pending) { pending = true; queueMicrotask(apply); } });
  let pending = false;

  function apply(): void {
    pending = false;
    observer.disconnect();
    document.documentElement.lang = language === 'zh' ? 'zh-CN' : language;
    document.title = translateText('firecal — A longer view of burning', language);
    const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
    let node: Node | null;
    while ((node = walker.nextNode())) {
      const text = node as Text;
      if (!text.parentElement || text.parentElement.closest('script, style, #language-select option, [data-no-translate], .maplibregl-ctrl-attrib')) continue;
      if (text.parentElement.matches('.calendar-grid .axis')) continue;
      const current = text.data;
      const prior = texts.get(text);
      const english = prior && current === prior.rendered ? prior.english : current;
      const rendered = translateText(english, language);
      texts.set(text, { english, rendered });
      if (current !== rendered) text.data = rendered;
    }
    root.querySelectorAll('[aria-label], [title], [placeholder], [alt]').forEach((element) => {
      let sources = attributes.get(element);
      if (!sources) { sources = new Map(); attributes.set(element, sources); }
      for (const name of ATTRIBUTES) {
        const current = element.getAttribute(name);
        if (current === null) continue;
        const prior = sources.get(name);
        const english = prior && current === prior.rendered ? prior.english : current;
        const rendered = translateText(english, language);
        sources.set(name, { english, rendered });
        if (current !== rendered) element.setAttribute(name, rendered);
      }
    });
    root.querySelectorAll('.calendar-grid').forEach((grid) => {
      [...grid.querySelectorAll('.axis')].slice(0, 12).forEach((axis, month) => {
        if (!axes.has(axis)) axes.set(axis, axis.textContent ?? '');
        const text = language === 'en' ? axes.get(axis)! : new Intl.DateTimeFormat(language, { month: 'short', timeZone: 'UTC' }).format(new Date(Date.UTC(2000, month, 1))).replace(/\.$/, '');
        axis.textContent = text;
      });
    });
    select!.value = language;
    observer.observe(root, { childList: true, subtree: true, characterData: true, attributes: true, attributeFilter: [...ATTRIBUTES] });
  }
  select.addEventListener('change', () => {
    if (!LANGUAGES.includes(select.value as Language)) return;
    language = select.value as Language;
    try { localStorage.setItem(KEY, language); } catch { /* Translation does not depend on storage. */ }
    apply();
  });
  apply();
}
