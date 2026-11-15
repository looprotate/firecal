import { translateText, type Language } from './translations.ts';
import type { MonthRange } from './period.ts';
import { monthCode } from './period.ts';

/** Serializes the rendered calendar with its actual colors and localized text into a standalone SVG. */
export function calendarImage(container: HTMLElement, areaLabel: string, period: MonthRange): string {
  const source=container.querySelector<SVGSVGElement>('.calendar-grid');
  if(!source)throw new Error('Load a calendar before exporting an image.');
  const ns='http://www.w3.org/2000/svg';
  const image=document.createElementNS(ns,'svg');image.setAttribute('xmlns',ns);
  const width=840, gridWidth=804, view=source.viewBox.baseVal, gridHeight=gridWidth*view.height/view.width;
  const height=Math.ceil(180+gridHeight);
  image.setAttribute('width',String(width));image.setAttribute('height',String(height));image.setAttribute('viewBox',`0 0 ${width} ${height}`);
  const rect=document.createElementNS(ns,'rect');rect.setAttribute('width','100%');rect.setAttribute('height','100%');rect.setAttribute('fill','#ffffff');image.append(rect);
  const language=document.documentElement.lang.slice(0,2) as Language;
  const text=(words:string,x:number,y:number,size:number):void=>{const t=document.createElementNS(ns,'text');t.setAttribute('x',String(x));t.setAttribute('y',String(y));t.setAttribute('font-family','Arial, sans-serif');t.setAttribute('font-size',String(size));t.setAttribute('fill','#141413');t.textContent=words;image.append(t);};
  text('firecal · '+translateText('Burning activity calendar',language),18,31,22);
  text(translateText(areaLabel,language),18,53,12);
  const mode=container.querySelector('[data-plane][aria-pressed="true"]')?.textContent??'';
  text(`${monthCode(period.first)} — ${monthCode(period.last)} · NASA FIRMS · MODIS + VIIRS · ${mode}`,18,73,12);
  const clone=source.cloneNode(true) as SVGSVGElement;
  const originals=[source,...source.querySelectorAll('*')],copies=[clone,...clone.querySelectorAll('*')];
  const properties=['fill','stroke','stroke-width','stroke-linecap','stroke-linejoin','opacity','visibility','font-family','font-size','font-weight'];
  originals.forEach((element,at)=>{const style=getComputedStyle(element),copy=copies[at]!;for(const property of properties)copy.setAttribute(property,style.getPropertyValue(property));});
  clone.removeAttribute('style');clone.setAttribute('x','18');clone.setAttribute('y','90');clone.setAttribute('width',String(gridWidth));clone.setAttribute('height',String(gridHeight));
  clone.querySelectorAll('.focus-ring').forEach(el=>el.setAttribute('visibility','hidden'));image.append(clone);
  const palette=getComputedStyle(document.documentElement);
  for(let step=0;step<8;step++){const swatch=document.createElementNS(ns,'rect');swatch.setAttribute('x',String(18+step*20));swatch.setAttribute('y',String(106+gridHeight));swatch.setAttribute('width','17');swatch.setAttribute('height','13');swatch.setAttribute('fill',palette.getPropertyValue(`--ramp-${step}`).trim());image.append(swatch);}
  text(translateText('Darker means more activity within this area and period.',language),190,117+gridHeight,12);
  text(translateText('Diamond: unusual month. Triangle: provisional records.',language),18,140+gridHeight,12);
  text(translateText('Hotspots are thermal anomalies, not measured fires or burned area.',language),18,161+gridHeight,11);
  const metadata=document.createElementNS(ns,'metadata');metadata.textContent='Source: NASA FIRMS. Harmonized MODIS-equivalent hotspots or fire radiative power. Thermal anomalies are not confirmed fires or burned area. Estimates are not forecasts.';image.append(metadata);
  return new XMLSerializer().serializeToString(image);
}
function save(blob: Blob,name:string):void {
  const url=URL.createObjectURL(blob);const link=document.createElement('a');link.href=url;link.download=name;link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);
}
export async function downloadCalendarImage(container:HTMLElement,label:string,period:MonthRange,format:'png'|'svg'):Promise<void> {
  const svg=calendarImage(container,label,period),blob=new Blob([svg],{type:'image/svg+xml;charset=utf-8'});
  const name=`firecal-${monthCode(period.first)}-${monthCode(period.last)}`;
  if(format==='svg'){save(blob,name+'.svg');return;}
  const url=URL.createObjectURL(blob),image=new Image();
  try {
    image.src=url;await image.decode();const canvas=document.createElement('canvas');canvas.width=image.naturalWidth*2;canvas.height=image.naturalHeight*2;
    const context=canvas.getContext('2d');if(!context)throw new Error('Image export is unavailable in this browser.');
    context.scale(2,2);context.drawImage(image,0,0);
    const png=await new Promise<Blob>((resolve,reject)=>canvas.toBlob(result=>result?resolve(result):reject(new Error('The image could not be exported.')),'image/png'));
    save(png,name+'.png');
  }finally{URL.revokeObjectURL(url);}
}
