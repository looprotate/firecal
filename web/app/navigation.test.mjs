import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { PROJECT_INFO } from './project-info.ts';
import { ANALYSIS_TOOLS } from './analysis-tools.ts';
import { SUPPORT_INFO } from './support.ts';
const shell = readFileSync(new URL('./shell.ts', import.meta.url), 'utf8');
const markup = shell + PROJECT_INFO + SUPPORT_INFO + ANALYSIS_TOOLS;
const links = [...markup.matchAll(/href="([^"]+)"/g)].map(match => match[1]);
test('the page has no repository links and only the primary explorer shortcut repeats', () => {
  assert.ok(links.length > 0);
  assert.ok(links.every(link => !/github/i.test(link)));
  const uniqueDestinations = links.filter(link => link !== '/explorer.html');
  assert.equal(new Set(uniqueDestinations).size, uniqueDestinations.length);
  assert.equal(links.filter(link => link === '/explorer.html').length, 2);
});
test('every internal navigation target is a section in the page', () => {
  for (const href of links.filter(link => link.startsWith('#'))) assert.ok(markup.includes(`id="${href.slice(1)}"`), href);
});
