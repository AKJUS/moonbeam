import "@moonbeam-network/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, WsProvider } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { generateKeyringPair } from "@moonwall/util";
import {
  XcmFragment,
  type RawXcmMessage,
  injectHrmpMessageAndSeal,
  descendOriginFromAddress20,
} from "../../../../helpers/xcm.js";

describeSuite({
  id: "D010714",
  title: "XCM - XcmPaymentApi - Transact",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    let polkadotJs: ApiPromise;
    let amountForFees: bigint;
    let amountForTransfer: bigint;
    let sendingAddress: `0x${string}`;
    let descendAddress: `0x${string}`;
    let random: KeyringPair;
    let balancesPalletIndex: number;
    const weightLimit = {
      refTime: 40_000_000_000n,
      proofSize: 150_000n,
    };
    let weightToNativeFee: any;

    beforeAll(async () => {
      polkadotJs = context.polkadotJs();

      // Get Pallet balances index
      const metadata = await polkadotJs.rpc.state.getMetadata();
      balancesPalletIndex = metadata.asLatest.pallets
        .find(({ name }) => name.toString() === "Balances")!
        .index.toNumber();

      // Fetch the exact amount of native fees that we will use given
      // the indicated weightLimit
      weightToNativeFee = await polkadotJs.call.xcmPaymentApi.queryWeightToAssetFee(weightLimit, {
        V3: {
          Concrete: {
            parents: 0,
            interior: {
              X1: { PalletInstance: balancesPalletIndex },
            },
          },
        },
      });

      expect(weightToNativeFee.isOk).to.be.true;

      const { originAddress, descendOriginAddress } = descendOriginFromAddress20(context);
      descendAddress = descendOriginAddress;
      sendingAddress = originAddress;

      random = generateKeyringPair();
      // Amount to use inside BuyExecution
      amountForFees = BigInt(weightToNativeFee.asOk.toJSON());
      // Amount to transfer to random address
      amountForTransfer = 1_000_000_000_000_000_000n;

      // We need to fund the descendAddress with both amounts.
      // This account takes care of paying the fees and also transfering the
      // tokens to the random address.
      await context.createBlock(
        polkadotJs.tx.balances.transferAllowDeath(
          descendAddress,
          amountForFees + amountForTransfer
        ),
        { allowFailures: false }
      );

      const balance = (await polkadotJs.query.system.account(descendAddress)).data.free.toBigInt();
      expect(balance).to.eq(amountForFees + amountForTransfer);
    });

    it({
      id: "T01",
      title: "Should de able to transact using the estimated native fees",
      test: async function () {
        // Build Transact encoded call
        const transferCall = polkadotJs.tx.balances.transferAllowDeath(
          random.address,
          amountForTransfer
        );
        const transferCallEncoded = transferCall?.method.toHex();

        // Build the XCM message with the corresponding weightLimit
        const xcmMessage = new XcmFragment({
          assets: [
            {
              multilocation: {
                parents: 0,
                interior: {
                  X1: { PalletInstance: balancesPalletIndex },
                },
              },
              fungible: amountForFees,
            },
          ],
          weight_limit: weightLimit,
          descend_origin: sendingAddress,
        })
          .descend_origin()
          .withdraw_asset()
          .buy_execution()
          .push_any({
            Transact: {
              originKind: "SovereignAccount",
              requireWeightAtMost: {
                refTime: 1000000000n,
                proofSize: 80000n,
              },
              call: {
                encoded: transferCallEncoded,
              },
            },
          })
          .as_v5();

        // Send an XCM and create block to execute it
        await injectHrmpMessageAndSeal(context, 1, {
          type: "XcmVersionedXcm",
          payload: xcmMessage,
        } as RawXcmMessage);

        // Make sure the random address received the transfer
        const testAccountBalance = (
          await polkadotJs.query.system.account(random.address)
        ).data.free.toBigInt();

        // Make sure the descendOrigin address has zero balance now
        const testDescendBalance = (
          await polkadotJs.query.system.account(descendAddress)
        ).data.free.toBigInt();

        expect(testAccountBalance).to.eq(amountForTransfer);
        expect(testDescendBalance).to.eq(0n);
      },
    });
  },
});
