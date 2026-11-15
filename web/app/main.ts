import { installScreens } from './screens.ts';
import { installBrandIntro } from './brand-intro.ts';
import { CONTINENTS, continentArea } from './continents.ts';
import { installMapExpansion } from './map-expansion.ts';
import { bindMapActivity } from './map-activity.ts';
import { installLanguages } from './languages.ts';
import { installAnalysisTools } from './analysis-tools.ts';
import { installComparison } from './comparison.ts';
import { loadCoverage, areaCoverage, type Coverage } from './coverage.ts';
import { queryRegion } from './query-region.ts';
import { parseSharedQuery, sharedUrl } from './share-query.ts';
import { downloadCalendarImage } from './calendar-image.ts';
import { regionLabel } from './region-label.ts';
import type { MonthRange } from './period.ts';
import type { PinnedAnswer } from './pinned-series.ts';
import '@watergis/maplibre-gl-terradraw/dist/maplibre-gl-terradraw.css';
import 'maplibre-gl/dist/maplibre-gl.css';
import '../tokens/tokens.css';
import './style.css';
import './refinements.css';
import './brand-intro.css';
import './analysis-tools.css';
import './theme.css';
import './screens.css';
import sample from '../stub/series.json';
import { mountCalendar, mountOverlapFigure, type CalendarActions } from '../calendar/index.ts';
import { loadFirecal, type Firecal } from '../../wasm/load.ts';
import { createAreaMap } from '../map/map.ts';
import { AREA_SELECTED_EVENT, areaFromSelectionEvent, emitAreaSelected, parseAreaBounds, polygonFromSelectionEvent, type AreaBoundsText } from './area-input.ts';
import { degreesText, latitudeLabel, longitudeLabel } from './degrees.ts';
import { pinRegion } from './offline.ts';
import { csvFiles } from './wasm-series.ts';
import { parseSeries, type Area, type Series } from './series.ts';
import { renderShell } from './shell.ts';

const root=document.querySelector<HTMLDivElement>('#app');
if(!root)throw new Error('The app root is missing');
const sampleSeries=parseSeries(sample),shared=parseSharedQuery(location.search);
const shell=renderShell(root);
installLanguages(root);
void installBrandIntro(root);
window.addEventListener('pageshow',event=>{if(event.persisted&&root)void installBrandIntro(root);});
const initialArea=shared?.area??{west:147,south:-38,east:151,north:-34};
const state:{area:Area;polygon?:[number,number][];series:Series;plane:Series['plane'];fromCore:boolean;actions:CalendarActions;bounds:AreaBoundsText;error:string;period?:MonthRange;month?:number;answer?:PinnedAnswer}={
  area:initialArea,polygon:shared?.polygon,series:sampleSeries,plane:shared?.plane??'count',fromCore:false,actions:{},bounds:boundsFromArea(initialArea),error:'',period:shared?.period,month:shared?.month,
};
let firecal:Firecal|undefined,coverage:Coverage|undefined,asked=0;
const calendar=mountCalendar(shell.calendarHost,state.series);
const areaMap=createAreaMap(shell.mapHost,selection=>emitAreaSelected(root,selection));
areaMap.showSelection({area:state.area,...(state.polygon?{polygon:state.polygon}:{})});
installMapExpansion(root,areaMap.map);
installScreens(root,()=>{areaMap.map.resize();areaMap.showSelection({area:state.area,...(state.polygon?{polygon:state.polygon}:{})});});
const analysis=installAnalysisTools(root,period=>{state.period=period;void answerFromPinnedTiles();},month=>mapActivity.refresh(month));
const mapActivity=bindMapActivity(root,areaMap,()=>state.fromCore&&state.series.area===state.area?state.series:null,month=>{state.month=month;analysis.summary(state.series,month);});
const comparison=installComparison(root,()=>({area:state.area,polygon:state.polygon,label:regionLabel(state.area)}),()=>firecal&&coverage?{core:firecal,coverage}:null,()=>({plane:state.plane,period:state.period}));

const continentPicker=root.querySelector<HTMLSelectElement>('#continent-select');
continentPicker?.addEventListener('change',()=>{const area=continentArea(continentPicker.value);if(area)emitAreaSelected(root,area);});
const connection=root.querySelector('#connection-state');
function updateConnection():void{if(connection)connection.textContent=navigator.onLine?'● Connected':'○ Offline · on this device';}
window.addEventListener('online',updateConnection);window.addEventListener('offline',updateConnection);updateConnection();
shell.areaForm.addEventListener('submit',event=>{
  event.preventDefault();state.bounds=readAreaBounds(shell.areaInputs);const result=parseAreaBounds(state.bounds);
  if(!result.area){state.error=result.error;render();return;}emitAreaSelected(root,result.area);
});
root.addEventListener(AREA_SELECTED_EVENT,event=>{
  const area=areaFromSelectionEvent(event);if(!area)return;
  mapActivity.clear();state.fromCore=false;state.answer=undefined;state.actions={};state.polygon=polygonFromSelectionEvent(event);state.area=area;state.bounds=boundsFromArea(area);state.error='';
  areaMap.showSelection({area,...(state.polygon?{polygon:state.polygon}:{})});render();void answerFromPinnedTiles();
});
shell.pinRegionButton.addEventListener('click',()=>void pinSelectedRegion());
root.querySelector('#share-query')!.addEventListener('click',()=>void shareCurrent());
for(const format of ['png','svg'] as const)root.querySelector<HTMLButtonElement>('#export-'+format)!.addEventListener('click',event=>void exportCurrent(format,event.currentTarget as HTMLButtonElement));
render();void loadRuntime();void showOverlapReport();
if('serviceWorker'in navigator)navigator.serviceWorker.register('/service-worker.js').catch((error:unknown)=>{shell.pinStatus.textContent=`Offline use is unavailable: ${error instanceof Error?error.message:String(error)}`;});

function render():void{
  for(const key of ['west','south','east','north'] as const)shell.areaInputs[key].value=state.bounds[key];
  shell.areaStatus.textContent=state.error;
  const {west,south,east,north}=state.area;
  shell.areaLabel.textContent=`${state.polygon?'Polygon in ':''}${latitudeLabel(south)} to ${latitudeLabel(north)}, ${longitudeLabel(west)} to ${longitudeLabel(east)}`;
  shell.dataSource.textContent=state.fromCore?'Harmonized tiles, answered on this device':'No fire data loaded for this area.';
  shell.dataSource.toggleAttribute('data-real',state.fromCore);
  calendar.update(state.series,state.actions);shell.calendarHost.hidden=!state.fromCore;
  const empty=root?.querySelector<HTMLElement>('#calendar-empty');if(empty)empty.hidden=state.fromCore;
  const hint=root?.querySelector<HTMLElement>('.interaction-hint');if(hint)hint.hidden=!state.fromCore;
  if(!state.fromCore)analysis.unavailable();
  const place=CONTINENTS.find(item=>{const box=continentArea(item.id)!;return !state.polygon&&Object.entries(box).every(([key,value])=>Math.abs(value-state.area[key as keyof Area])<0.0001);});
  if(continentPicker)continentPicker.value=place?.id??'';
}
function readAreaBounds(inputs:Record<keyof AreaBoundsText,HTMLInputElement>):AreaBoundsText{return{west:inputs.west.value,south:inputs.south.value,east:inputs.east.value,north:inputs.north.value};}
function boundsFromArea(area:Area):AreaBoundsText{return{west:degreesText(area.west),south:degreesText(area.south),east:degreesText(area.east),north:degreesText(area.north)};}
async function loadRuntime():Promise<void>{
  try{
    const response=await fetch('/firecal.wasm');if(!response.ok)throw new Error('The query engine could not be loaded.');
    [firecal,coverage]=await Promise.all([loadFirecal(await response.arrayBuffer()),loadCoverage()]);
    shell.runtimeStatus.textContent='Loading the available region…';await answerFromPinnedTiles();
  }catch(error){shell.runtimeStatus.textContent=error instanceof Error?error.message:'The core could not be loaded.';}
}
async function pinSelectedRegion():Promise<void>{
  if(!coverage||!areaCoverage(state.area,coverage)){shell.pinStatus.textContent='No downloadable data for this area yet.';return;}
  shell.pinRegionButton.disabled=true;shell.pinStatus.textContent='Saving the tiles of this region…';
  try{const report=await pinRegion(state.area);shell.pinStatus.textContent=`Pinned ${report.tiles} tiles, ${(report.bytes/1024/1024).toFixed(2)} MiB.`;await answerFromPinnedTiles();}
  catch(error){shell.pinStatus.textContent=error instanceof Error?error.message:'The region could not be pinned.';}
  finally{shell.pinRegionButton.disabled=false;}
}
async function answerFromPinnedTiles():Promise<void>{
  const ticket=++asked;if(!firecal||!coverage)return;
  shell.runtimeStatus.textContent='Loading the available region…';
  try{
    const answer=await queryRegion(firecal,coverage,{area:state.area,plane:state.plane,period:state.period,...(state.polygon?{polygon:state.polygon}:{})});
    if(ticket!==asked)return;
    if(!answer){state.fromCore=false;state.answer=undefined;state.actions={};render();mapActivity.clear();shell.runtimeStatus.textContent='No fire data is available for this area yet.';return;}
    state.answer=answer;state.series=answer.series;state.period={first:answer.period.first,last:answer.period.last};state.fromCore=true;
    state.actions={choosePlane:plane=>{state.plane=plane;void answerFromPinnedTiles();},csv:csvFiles(firecal,answer.query,state.polygon!==undefined,answer.period)};
    render();state.month=analysis.configure(answer,state.month);mapActivity.refresh(state.month);comparison.refresh();
    shell.runtimeStatus.textContent=`From the core, on ${answer.tiles} pinned tiles, ${answer.series.months.length} months.`;
  }catch(error){if(ticket!==asked)return;state.fromCore=false;state.answer=undefined;render();mapActivity.clear();shell.runtimeStatus.textContent=error instanceof Error?error.message:'The pinned tiles could not be read.';}
}
async function shareCurrent():Promise<void>{
  if(!state.fromCore||!state.answer)return;
  try { const url=sharedUrl(location.href,{area:state.area,polygon:state.polygon,plane:state.plane,period:state.period,month:state.month});analysis.link(url);
    try{if(!navigator.clipboard)throw new Error();await navigator.clipboard.writeText(url);analysis.status('Query link copied.');}
    catch{analysis.status('Copy the link from the field below.');}
  }catch(error){analysis.status(error instanceof Error?error.message:'This query cannot be shared.');}
}
async function exportCurrent(format:'png'|'svg',button:HTMLButtonElement):Promise<void>{
  if(!state.fromCore||!state.answer)return;button.disabled=true;analysis.status('Preparing the calendar image…');
  try{await downloadCalendarImage(shell.calendarHost,regionLabel(state.area),state.answer.period,format);analysis.status('Calendar image downloaded.');}
  catch(error){analysis.status(error instanceof Error?error.message:'The image could not be exported.');}
  finally{button.disabled=false;}
}
async function showOverlapReport():Promise<void>{
  try{const response=await fetch('/tiles/overlap.csv');const text=response.ok?await response.text():'';if(!text.startsWith('tile_row,'))return;mountOverlapFigure(shell.overlapHost,text);shell.harmonization.hidden=false;}catch{/* The figure is optional when offline. */}
}
