import { StorageKey } from '@polkadot/types'
import { Codec } from '@polkadot/types/types'
import { blake2AsHex } from '@polkadot/util-crypto'
import _ from 'lodash'

export const GETH = 5
export const KSM = 4
export const BASE_TOKEN = process.env.APP_ENV === 'rollup-testnet' ? GETH : KSM

export type CodecOrArray = Codec | Codec[]

export const processCodecOrArray = (
  codec: CodecOrArray,
  fn: (c: Codec) => any
) => (Array.isArray(codec) ? codec.map(fn) : fn(codec))

export const toHuman = (codec: CodecOrArray) =>
  processCodecOrArray(codec, (c) => c?.toHuman?.() ?? c)

export const parseNumber = (n: string | number) =>
  _.isNumber(n) ? n : Number.parseInt(n.replace(/,/g, ''))

export const parseKey = (key: StorageKey) => key.toString().substring(66)

export const blakexx = (n: Codec) => blake2AsHex(n.toHex()).substring(2)
