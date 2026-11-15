import test from 'node:test';import assert from 'node:assert/strict';import {areaCoverage,parseCoverage} from './coverage.ts';
const coverage={version:1,tiles:[{row:12,col:32,firstMonth:10,months:230},{row:12,col:33,firstMonth:144,months:96}]};
test('coverage is the common period of every tile the area touches',()=>{assert.deepEqual(areaCoverage({west:147,south:-38,east:151,north:-34},coverage),{first:144,last:239,tiles:2});});
test('one known tile never makes an unknown neighbor look like zero fire',()=>{assert.equal(areaCoverage({west:147,south:-38,east:151,north:-34},{version:1,tiles:coverage.tiles.slice(0,1)}),null);});
test('duplicate or malformed catalog entries cannot authorize a query',()=>{assert.throws(()=>parseCoverage({tiles:[coverage.tiles[0],coverage.tiles[0]]}));assert.throws(()=>parseCoverage({tiles:[{row:30,col:32,firstMonth:0,months:12}]}));});
