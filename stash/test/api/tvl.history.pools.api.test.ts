import { describe, expect, it } from 'vitest'
import supertest from 'supertest'
import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL } from './utils'

describe('APi tests: tvl-history/pools', () => {
  const testPool = '0-2'
  const testPoolReversed = '2-0'
  it('GET pools/0-2 returns the same as pools/2-0 -> Expect deep equal', async () => {
    const gaspv2L1Asset = await supertest(app)
      .get('/tvl-history/pools/' + testPool)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    const l1AssetGaspv2 = await supertest(app)
      .get('/tvl-history/pools/' + testPoolReversed)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    expect(gaspv2L1Asset.body).to.deep.equal(l1AssetGaspv2.body)
    expect(gaspv2L1Asset.body).to.have.property('volumes')
    expect(gaspv2L1Asset.body.volumes).to.be.an('array')
    expect(gaspv2L1Asset.body.volumes[0]).to.have.a.lengthOf(2)
  })
})

// These tests will fail if images changes And/Or if bugfixes. Careful when updating!
describe.todo('Snapshots tests: tvl-history/pools', () => {
  //more tests will come...
})
describe('API Errors: tvl-history/pools', () => {
  it('GET pools/foo does not exist Expect validation error', async () => {
    const errorMessage =
      'this must be one of the following values: 0-2, 2-0, 4-0, 0-4, 0-7, 7-0, 4-7, 7-4, 5-0, 0-5, 0-11, 11-0, 11-4, 4-11, 0-14, 14-0, 16-0, 0-16, 16-4, 4-16, ALL'
    await supertest(app)
      .get('/tvl-history/pools/foo')
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(500)
      .then((response) => {
        const fooResponse = response.body
        expect(fooResponse.exceptionName).to.contain('ValidationError')
        expect(fooResponse.message).to.contain(errorMessage)
      })
  })
})
describe.todo('System Errors: tvl-history/pools', () => {
  //more tests will come...
})
