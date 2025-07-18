import "@moonbeam-network/api-augment";
import { afterEach, beforeAll, describeSuite, expect } from "@moonwall/cli";

import { sovereignAccountOfSibling, sendCallAsPara } from "../../../../helpers/xcm.js";
import { fundAccount } from "../../../../helpers/balances.js";
import { expectSubstrateEvent, expectSystemEvent } from "../../../../helpers/expect.js";

describeSuite({
  id: "D020115",
  title: "Freezing and Unfreezing Foreign Assets via XCM",
  foundationMethods: "dev",
  testCases: ({ context, it }) => {
    const fundAmount = 100_000_000_000_000_000_000_000n;
    const assetId = 7;
    const assetLocation = {
      parents: 1,
      interior: {
        X3: [{ Parachain: 3000 }, { PalletInstance: 5 }, { GeneralIndex: 5 }],
      },
    };

    beforeAll(async () => {
      // Sibling Paras
      const siblingParas = [3000, 4000];
      const siblingParaSovereignAccounts = siblingParas.map((paraId) =>
        sovereignAccountOfSibling(context, paraId)
      );

      // Fund all accounts
      const fundAmount = 100_000_000_000_000_000_000_000n;
      for (const address of siblingParaSovereignAccounts) {
        await fundAccount(address as `0x${string}`, fundAmount, context);
      }

      // Create a foreign asset
      const createForeignAssetCall = context
        .polkadotJs()
        .tx.evmForeignAssets.createForeignAsset(assetId, assetLocation, 18, "TEST", "TEST");
      const { blockRes } = await sendCallAsPara(
        createForeignAssetCall,
        3000,
        context,
        fundAmount / 20n
      );
      await expectSystemEvent(
        blockRes.block.hash,
        "evmForeignAssets",
        "ForeignAssetCreated",
        context
      );
    });

    afterEach(async () => {
      // Reset asset state and expect it to be active
      const assetByLocation = (
        await context.polkadotJs().query.evmForeignAssets.assetsByLocation(assetLocation)
      ).toJSON();
      if (assetByLocation![1] !== "Active") {
        const unfreezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.unfreezeForeignAsset(assetId);
        const sudoCall = context.polkadotJs().tx.sudo.sudo(unfreezeForeignAssetCall);
        const block = await context.createBlock(sudoCall);
        await expectSubstrateEvent(block, "evmForeignAssets", "ForeignAssetUnfrozen");
      }
      const assetAfter = (
        await context.polkadotJs().query.evmForeignAssets.assetsByLocation(assetLocation)
      ).toJSON();
      expect(assetAfter![1]).to.eq("Active");
    });

    it({
      id: "T01",
      title: "Should not be able to freeze/unfreeze if already frozen via XCM",
      test: async function () {
        const freezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.freezeForeignAsset(assetId, false);

        const { blockRes: block1 } = await sendCallAsPara(
          freezeForeignAssetCall,
          3000,
          context,
          fundAmount / 20n
        );
        await expectSystemEvent(
          block1.block.hash,
          "evmForeignAssets",
          "ForeignAssetFrozen",
          context
        );

        const { errorName } = await sendCallAsPara(
          freezeForeignAssetCall,
          3000,
          context,
          fundAmount / 20n,
          true
        );
        expect(errorName).to.eq("AssetAlreadyFrozen");

        const unfreezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.unfreezeForeignAsset(assetId);

        const { blockRes: block3 } = await sendCallAsPara(
          unfreezeForeignAssetCall,
          3000,
          context,
          fundAmount / 20n
        );
        await expectSystemEvent(
          block3.block.hash,
          "evmForeignAssets",
          "ForeignAssetUnfrozen",
          context
        );

        const { errorName: error2 } = await sendCallAsPara(
          unfreezeForeignAssetCall,
          3000,
          context,
          fundAmount / 20n,
          true
        );
        expect(error2).to.eq("AssetNotFrozen");
      },
    });

    it({
      id: "T03",
      title: "Should not be able to freeze/unfreeze if asset does not exist",
      test: async function () {
        const freezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.freezeForeignAsset(255, false);

        const { errorName: error1 } = await sendCallAsPara(
          freezeForeignAssetCall,
          3000,
          context,
          fundAmount / 20n,
          true
        );
        expect(error1).to.eq("AssetDoesNotExist");

        const unfreezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.unfreezeForeignAsset(255);

        const { errorName: error2 } = await sendCallAsPara(
          unfreezeForeignAssetCall,
          3000,
          context,
          fundAmount / 20n,
          true
        );
        expect(error2).to.eq("AssetDoesNotExist");
      },
    });

    it({
      id: "T04",
      title: "Should not be able to freeze/unfreeze if not owner",
      test: async function () {
        const freezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.freezeForeignAsset(assetId, false);

        const { errorName: error1 } = await sendCallAsPara(
          freezeForeignAssetCall,
          4000,
          context,
          fundAmount / 20n,
          true
        );
        expect(error1).to.eq("LocationOutsideOfOrigin");

        const unfreezeForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.unfreezeForeignAsset(assetId);

        const { errorName: error2 } = await sendCallAsPara(
          unfreezeForeignAssetCall,
          4000,
          context,
          fundAmount / 20n,
          true
        );
        expect(error2).to.eq("LocationOutsideOfOrigin");
      },
    });
  },
});
