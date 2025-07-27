import test from 'ava'

import { ping } from '../index'

test('ping should return success for a valid domain', async (t) => {
  const ret = await ping('8.8.8.8');
  console.log(ret)
  t.is(ret.success, true)
})

test('ping should return success for a valid ipv6 address', async (t) => {
  const ret = await ping('2001:4860:4860::8888');
  console.log(ret)
  t.is(ret.success, true)
});
