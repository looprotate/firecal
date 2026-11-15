import { mountCalendar, type Calendar } from '../calendar/mount.ts';
import { areaCoverage, type Coverage } from './coverage.ts';
import { queryRegion } from './query-region.ts';
import { resolvePeriod, type MonthRange } from './period.ts';
import type { Firecal } from '../../wasm/load.ts';
import type { Area, Series } from './series.ts';
export interface Zone { area: Area; polygon?: [number,number][]; label: string }
export function sameZone(a:Zone,b:Zone):boolean {return Object.keys(a.area).every(key=>a.area[key as keyof Area]===b.area[key as keyof Area])&&JSON.stringify(a.polygon??[])===JSON.stringify(b.polygon??[]);}

/** Side-by-side queries share both period, baseline years and color scale. */
export function installComparison(root:HTMLElement, current:()=>Zone, runtime:()=>{core:Firecal;coverage:Coverage}|null, view:()=>{plane:Series['plane'];period?:MonthRange}):{refresh():void} {
  let zoneA:Zone|null=null;let shown=false;let serial=0;let calendars:Calendar[]=[];
  const status=root.querySelector<HTMLOutputElement>('#comparison-status')!,board=root.querySelector<HTMLElement>('#comparison-board')!;
  const clear=():void=>{calendars.forEach(calendar=>calendar.destroy());calendars=[];board.replaceChildren();};
  root.querySelector('#capture-zone')!.addEventListener('click',()=>{
    const zone=current();zoneA=structuredClone(zone);shown=false;serial++;clear();
    status.textContent='Zone A saved. Select a different area for zone B.';
  });
  const compare=async():Promise<void>=>{
    const ticket=++serial;const active=runtime(),zoneB=current();
    if(!zoneA){status.textContent='Save zone A first.';return;}
    if(sameZone(zoneA,zoneB)){status.textContent='Select a different area for zone B.';return;}
    if(!active){status.textContent='Load the available data first.';return;}
    const a=areaCoverage(zoneA.area,active.coverage),b=areaCoverage(zoneB.area,active.coverage);
    if(!a||!b){clear();status.textContent='Both areas need complete published coverage.';return;}
    const first=Math.max(a.first,b.first),last=Math.min(a.last,b.last);
    if(first>last){clear();status.textContent='The areas have no shared period.';return;}
    const period=resolvePeriod(view().period,first,last),baselineFirst=2000+Math.ceil(first/12);
    status.textContent='Comparing the two areas…';
    try {
      const question={plane:view().plane,period,baselineFirst};
      const [left,right]=await Promise.all([queryRegion(active.core,active.coverage,{...zoneA,...question}),queryRegion(active.core,active.coverage,{...zoneB,...question})]);
      if(ticket!==serial)return;
      if(!left||!right)throw new Error('Both areas need complete published coverage.');
      clear();const max=Math.max(...left.series.value,...right.series.value,1);
      for(const [zone,answer,label]of [[zoneA,left,'A'],[zoneB,right,'B']] as const){
        const panel=document.createElement('section'),title=document.createElement('h3'),subtitle=document.createElement('p'),host=document.createElement('div');
        panel.className='comparison-panel';title.textContent=`${label} · ${zone.label}`;
        const year=2000+Math.floor(period.last/12);
        const reference=answer.query.baselineLast<year?`Reference period: ${answer.query.baselineFirst}–${answer.query.baselineLast}`:'No historical baseline is available for this year.';
        subtitle.textContent=`${2000+Math.floor(period.first/12)}–${year} · ${view().plane==='count'?'Hotspots':'Radiative power'} · ${reference}`;
        panel.append(title,subtitle,host);board.append(panel);calendars.push(mountCalendar(host,{...answer.series,scale_max:max}));
      }
      shown=true;status.textContent='Both calendars use the same color scale, period and reference years. Totals are not normalized by area size.';
    }catch(error){if(ticket===serial){clear();status.textContent=error instanceof Error?error.message:'The comparison could not be loaded.';}}
  };
  root.querySelector('#compare-zones')!.addEventListener('click',()=>void compare());
  return{refresh(){if(shown)void compare();}};
}
