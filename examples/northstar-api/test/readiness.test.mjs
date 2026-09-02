import test from 'node:test';
import assert from 'node:assert/strict';
import { bearing } from '../src/index.ts';

for (let index = 0; index < 42; index += 1) {
  test(`northstar bearing check ${index + 1}`, () => {
    assert.equal(bearing(index, index + 90), 90);
  });
}
