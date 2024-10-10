import { describe, beforeAll, expect, it , vi} from "vitest";
import { L2Interface } from "../src/l2";
import { hexToU8a } from "@polkadot/util";
import { Deposit } from "../src/common/deposit.js";
import { L1Interface } from "../src/l1/index.js";
import { Ferry } from "../src/ferry/index.js";


const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";
const NATIVE_TOKEN = hexToU8a("0x0000000000000000000000000000000000000001", 160);
const FERRY_TOKEN1 = hexToU8a("0x1111111111111111111111111111111111111111", 160);
const FERRY_TOKEN2 = hexToU8a("0x2222222222222222222222222222222222222222", 160);
const DUMMY_TOKEN = hexToU8a("0x3333333333333333333333333333333333333333", 160);
const DEFAULT_BALANCE = 1000000n;
const TOKENS_TO_FERRY: [Uint8Array, bigint, bigint][] = [
  [NATIVE_TOKEN, 10000n, 1n],
  [FERRY_TOKEN1, 10000n, 1n],
  [FERRY_TOKEN2, 20000n, 1n],
];

let ferry: Ferry;
let l1Mock: L1Interface;
let l2Mock: L2Interface;

describe('Ferry Service', () => {

  beforeAll(async () => {

    l1Mock = {
      isRolldownDeployed: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      getFerriedAndReadyToClose: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      getNativeTokenAddress: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      getLatestRequestId: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      isClosed: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      isFerried: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      ferry: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      close: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called")}),
      getBalance: vi.fn().mockImplementation(() => DEFAULT_BALANCE),
    };

    l2Mock = {
        getLatestRequestId: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getWithdrawals: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getNativeTokenAddress: vi.fn().mockImplementation(() => NATIVE_TOKEN),
    };

    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, TOKENS_TO_FERRY, 0n);
  });

  it('should use latests requests ids to fetch pending withdrawals', async () => {

    const latestRequestIdL1 = 123n;
    const latestRequestIdL2 = 456n;
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(latestRequestIdL1);
    l2Mock.getLatestRequestId = vi.fn().mockResolvedValue(latestRequestIdL2);

    l2Mock.getWithdrawals = vi.fn()
      .mockImplementationOnce(async (arg1, arg2) => {
        if (arg1 === latestRequestIdL1 && arg2 === latestRequestIdL2) {
          return [];
        }
        throw new Error("Unexpected arguments");
      });
    const withdrawals = await ferry.getPendingWithdrawals();
    expect(withdrawals).toHaveLength(0);
  });

  it('should use return ampty array when one of counter does not exist 1', async () => {

    const latestRequestIdL1 = null;
    const latestRequestIdL2 = 456n;
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(latestRequestIdL1);
    l2Mock.getLatestRequestId = vi.fn().mockResolvedValue(latestRequestIdL2);

    const withdrawals = await ferry.getPendingWithdrawals();
    expect(withdrawals).toHaveLength(0);
  });

  it('should use return ampty array when one of counter does not exist 2', async () => {

    const latestRequestIdL1 = 456n;
    const latestRequestIdL2 = null;
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(latestRequestIdL1);
    l2Mock.getLatestRequestId = vi.fn().mockResolvedValue(latestRequestIdL2);

    const withdrawals = await ferry.getPendingWithdrawals();
    expect(withdrawals).toHaveLength(0);
  });

  it('should fetch withdrawals that are not ferried and not closed', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(1n);
    l2Mock.getLatestRequestId = vi.fn().mockResolvedValue(2n);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue(
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      }]
    );
    const withdrawals = await ferry.getPendingWithdrawals();
    expect(withdrawals).toHaveLength(1);
  });

  it('getWithdrawals should ignore ferried txs', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(1n);
    l2Mock.getLatestRequestId = vi.fn().mockResolvedValue(2n);
    l1Mock.isFerried = vi.fn().mockResolvedValue(true);
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue(
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      }]
    );
    const withdrawals = await ferry.getPendingWithdrawals();
    expect(withdrawals).toHaveLength(0);
  });

  it('getWithdrawals should ignore closed txs', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(1n);
    l2Mock.getLatestRequestId = vi.fn().mockResolvedValue(2n);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);
    l1Mock.isClosed = vi.fn().mockResolvedValue(true);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue(
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      }]
    );
    const withdrawals = await ferry.getPendingWithdrawals();
    expect(withdrawals).toHaveLength(0);
  });

  it('rateWithdrawals considers only tokens passed in ctor', async () => {

    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
        {
        requestId: 2n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: DUMMY_TOKEN,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      }];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(1);
    expect(ferryableWithdrawals[0]).to.be.equal(withdrawals[0]);
  });


  it('rateWithdrawals considers ignores withdrawals that can not afford', async () => {
    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: DEFAULT_BALANCE + 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
        {
        requestId: 2n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: DEFAULT_BALANCE,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      }];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(1);
    expect(ferryableWithdrawals[0]).to.be.equal(withdrawals[1]);
  });

  it('rateWithdrawals considers tx cost for erc ferries', async () => {
    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: DEFAULT_BALANCE + 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
        {
        requestId: 2n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount: DEFAULT_BALANCE,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      }];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(1);
    expect(ferryableWithdrawals[0]).to.be.equal(withdrawals[1]);
  });

  it('rateWithdrawals considers tx cost for native withdrawals', async () => {

    const MIN_ACCOUNT_BALANCE = 1000n;
    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, TOKENS_TO_FERRY, MIN_ACCOUNT_BALANCE);
    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: NATIVE_TOKEN,
        amount:  DEFAULT_BALANCE,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
      ];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(0);
  });

  it('rateWithdrawals ignores tx cost for erc20 withdrawals', async () => {

    const MIN_ACCOUNT_BALANCE = 1000n;
    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, TOKENS_TO_FERRY, MIN_ACCOUNT_BALANCE);
    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount:  DEFAULT_BALANCE,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
      ];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(1);
    expect(ferryableWithdrawals[0]).to.be.equal(withdrawals[0]);
  });

  it('rateWithdrawals considers assigned weight', async () => {
    const tokensToFerry: [Uint8Array, bigint, bigint][] = [
      [FERRY_TOKEN1, 10000n, 1n],
      [FERRY_TOKEN2, 20000n, 2n],
    ];
    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, tokensToFerry, 0n);
    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount:  DEFAULT_BALANCE,
        ferryTip: 1n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
      {
        requestId: 2n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN2,
        amount:  DEFAULT_BALANCE,
        ferryTip: 1n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
      ];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(2);
    expect(ferryableWithdrawals[0].requestId).to.be.equal(2n);
  });

  it('rateWithdrawals considers amount when weight is the same assigned weight', async () => {
    const withdrawals = 
      [{
        requestId: 1n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount:  DEFAULT_BALANCE,
        ferryTip: 1n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
      {
        requestId: 2n,
        withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
        tokenAddress: FERRY_TOKEN1,
        amount:  30000n,
        ferryTip: 1n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
      },
      ];

    const ferryableWithdrawals = await ferry.rateWithdrawals(withdrawals);
    expect(ferryableWithdrawals).toHaveLength(2);
    expect(ferryableWithdrawals[0].requestId).to.be.equal(1n);
  });
})
