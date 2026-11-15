import test from 'node:test';
import assert from 'node:assert/strict';
import { installNavigationDrawer } from './navigation-drawer.ts';
function setup(t) {
 const savedDocument=globalThis.document,savedMedia=globalThis.matchMedia;
 const toggle=new EventTarget(),dialog=new EventTarget(),closeButton=new EventTarget(),media=new EventTarget();
 const attributes=new Map();let location='header';
 toggle.setAttribute=(key,value)=>attributes.set(key,value);closeButton.focus=()=>{};
 const navigation={before(){}};
 const slot={append(){location='drawer';}};
 dialog.open=false;dialog.showModal=()=>{dialog.open=true;};
 dialog.close=()=>{dialog.open=false;dialog.dispatchEvent(new Event('close'));};
 dialog.getBoundingClientRect=()=>({left:600,right:1000,top:0,bottom:800});
 dialog.querySelector=selector=>selector==='.navigation-drawer-content'?slot:closeButton;
 const root={querySelector:selector=>({'.menu-toggle':toggle,'#main-navigation':navigation,'#navigation-drawer':dialog})[selector]};
 globalThis.document={createComment:()=>({after(){location='header';}}),body:{style:{overflow:'auto'}}};
 globalThis.matchMedia=()=>media;
 const savedWindow=globalThis.window;globalThis.window=new EventTarget();
 t.after(()=>{globalThis.document=savedDocument;globalThis.matchMedia=savedMedia;globalThis.window=savedWindow;});
 const drawer=installNavigationDrawer(root);
 return {toggle,dialog,closeButton,media,drawer,attributes,location:()=>location};
}
test('the hamburger moves the existing navigation into a modal and restores it on selection',t=>{
 const ui=setup(t);ui.toggle.dispatchEvent(new Event('click'));
 assert.equal(ui.dialog.open,true);assert.equal(ui.location(),'drawer');assert.equal(document.body.style.overflow,'hidden');assert.equal(ui.attributes.get('aria-expanded'),'true');
 ui.drawer.close();assert.equal(ui.dialog.open,false);assert.equal(ui.location(),'header');assert.equal(document.body.style.overflow,'auto');assert.equal(ui.attributes.get('aria-expanded'),'false');
});
test('Escape, the close button, outside click and desktop resize restore the closed state',t=>{
 const ui=setup(t);
 for(const action of [()=>ui.dialog.dispatchEvent(new Event('cancel',{cancelable:true})),()=>ui.closeButton.dispatchEvent(new Event('click')),()=>ui.dialog.dispatchEvent(Object.assign(new Event('click'),{clientX:400,clientY:200})),()=>ui.media.dispatchEvent(Object.assign(new Event('change'),{matches:false}))]){
  ui.toggle.dispatchEvent(new Event('click'));action();assert.equal(ui.dialog.open,false);assert.equal(ui.location(),'header');assert.equal(document.body.style.overflow,'auto');
 }
});
