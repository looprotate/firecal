import { monthCode, parseMonth, type MonthRange } from './period.ts';
import { formatNumber, monthTitle } from '../calendar/labels.ts';
import type { PinnedAnswer } from './pinned-series.ts';
import type { Series } from './series.ts';

export const ANALYSIS_TOOLS = `<div class="analysis-tools" id="analysis-tools" hidden>
  <form id="period-form" class="period-form"><label>From<input type="month" id="period-from" required></label><label>To<input type="month" id="period-to" required></label><button type="submit">Apply period</button><button type="button" id="period-reset">All available years</button></form>
  <p class="analysis-note" id="baseline-note"></p><output class="analysis-status" id="analysis-status" aria-live="polite"></output>
  <section class="month-summary" aria-labelledby="month-summary-title"><div class="month-summary-top"><h3 id="month-summary-title">Selected month</h3><label class="summary-month-label"><span class="sr-only">Selected month</span><input type="month" id="summary-month"></label></div><div class="summary-metrics"><div><span id="summary-unit">Hotspots</span><strong id="summary-value">—</strong></div><div><span>Historical median</span><strong id="summary-median">—</strong></div><div><span>Difference from median</span><strong id="summary-change">—</strong></div><div><span>Estimated uncertainty</span><strong id="summary-sigma">—</strong></div></div><p id="summary-verdict"></p></section>
  <div class="analysis-actions"><button type="button" id="share-query">Copy query link ↗</button><button type="button" id="export-png">Download PNG ↓</button><button type="button" id="export-svg">Download SVG ↓</button></div>
  <label class="shared-link-label" id="shared-link-label" hidden>Query link<input readonly id="shared-link"></label>
  <details class="comparison-tools" id="comparison-details"><summary>Compare two areas</summary><p>Save the current area as zone A, select a different area on the map, then compare it as zone B.</p><div class="analysis-actions"><button type="button" id="capture-zone">Save this area as A</button><button type="button" id="compare-zones">Compare with this area as B</button></div><output id="comparison-status" aria-live="polite"></output><div id="comparison-board"></div></details>
</div>`;

export interface AnalysisControls {
  configure(answer: PinnedAnswer, selectedMonth?: number): number;
  summary(series: Series, month: number): void;
  status(message: string): void;
  link(url: string): void;
  unavailable(): void;
}
/** Reads the core's already-computed statistics; only display formatting happens here. */
export function installAnalysisTools(root: HTMLElement, onPeriod: (range?: MonthRange) => void, onMonth: (month: number) => void): AnalysisControls {
  const host=root.querySelector<HTMLElement>('#analysis-tools')!;
  const from=root.querySelector<HTMLInputElement>('#period-from')!,to=root.querySelector<HTMLInputElement>('#period-to')!,selected=root.querySelector<HTMLInputElement>('#summary-month')!;
  const output=(id:string,text:string):void=>{const el=root.querySelector<HTMLElement>('#'+id);if(el)el.textContent=text;};
  const status=(message:string):void=>output('analysis-status',message);
  root.querySelector('#period-form')!.addEventListener('submit',event=>{
    event.preventDefault();const first=parseMonth(from.value),last=parseMonth(to.value);
    if(first===null||last===null||first>last){status('The start month must not be after the end month.');return;}
    onPeriod({first,last});
  });
  root.querySelector('#period-reset')!.addEventListener('click',()=>onPeriod(undefined));
  selected.addEventListener('change',()=>{const month=parseMonth(selected.value);if(month!==null&&selected.checkValidity())onMonth(month);else status('Choose a month within the displayed period.');});
  const summary=(series:Series,month:number):void=>{
    const at=month-series.first_month_index;if(at<0||at>=series.value.length)return;
    selected.value=monthCode(month);
    output('month-summary-title',monthTitle(2000+Math.floor(month/12),month%12+1));
    output('summary-unit',series.plane==='energy'?'Radiative power · MW':'Hotspots');
    output('summary-value',formatNumber(series.value[at]!));
    const median=series.median[month%12]??NaN,sigma=series.sigma[at]??NaN,change=series.anomaly[at]??NaN;
    output('summary-median',Number.isFinite(median)?formatNumber(median):'—');
    output('summary-change',Number.isFinite(change)&&median>=1?`${change>=0?'+':'−'}${formatNumber(Math.abs(change)*100)}%`:'—');
    output('summary-sigma',Number.isFinite(sigma)?`±${new Intl.NumberFormat('en-US',{maximumFractionDigits:1}).format(sigma)} (estimated)`:'—');
    output('summary-verdict',!Number.isFinite(median)?'No historical baseline is available for this year.':!Number.isFinite(sigma)?'No uncertainty estimate is available in radiative power mode.':series.unusual[at]?'Unusual: outside the uncertainty band and historical range.':'Within the usual range.');
  };
  return {
    configure(answer,month){
      host.hidden=false;from.min=to.min=monthCode(answer.available.first);from.max=to.max=monthCode(answer.available.last);
      from.value=monthCode(answer.period.first);to.value=monthCode(answer.period.last);
      selected.min=from.value;selected.max=to.value;
      const year=2000+Math.floor(answer.period.last/12);
      output('baseline-note',answer.query.baselineLast<year?`Reference period: ${answer.query.baselineFirst}–${answer.query.baselineLast}. Earlier records remain available for historical context.`:'No historical baseline is available for this year.');
      status(answer.adjusted?'The period was adjusted to the available record.':'');
      const actual=month!==undefined&&month>=answer.period.first&&month<=answer.period.last?month:answer.period.last;
      summary(answer.series,actual);return actual;
    },summary,status,
    link(url){root.querySelector<HTMLElement>('#shared-link-label')!.hidden=false;root.querySelector<HTMLInputElement>('#shared-link')!.value=url;},
    unavailable(){host.hidden=true;status('');},
  };
}
