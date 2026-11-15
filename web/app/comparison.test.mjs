import test from 'node:test';import assert from 'node:assert/strict';import {sameZone} from './comparison.ts';import {buildGrid} from '../calendar/grid.ts';
const a={area:{west:0,south:0,east:1,north:1},label:'A'};
test('comparison distinguishes actual geometry rather than zone labels',()=>{assert.equal(sameZone(a,{...a,label:'Different label'}),true);assert.equal(sameZone(a,{...a,area:{...a.area,east:2}}),false);assert.equal(sameZone(a,{...a,polygon:[[0,0],[1,1],[0,1]]}),false);});
test('comparison calendars use a common color scale',()=>{const s={plane:'count',first_month_index:0,value:[25],sigma:[1]};assert.equal(buildGrid(s).rows[0][0].step,7);assert.equal(buildGrid({...s,scale_max:100}).rows[0][0].step,4);});
