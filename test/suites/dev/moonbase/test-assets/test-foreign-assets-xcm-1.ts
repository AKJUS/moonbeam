import "@moonbeam-network/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { sendCallAsPara, sovereignAccountOfSibling } from "../../../../helpers/xcm.js";
import { fundAccount } from "../../../../helpers/balances.js";
import { expectSystemEvent } from "../../../../helpers/expect.js";

describeSuite({
  id: "D020108",
  title: "Create & manage Foreign Assets via XCM",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    const fundAmount = 100_000_000_000_000_000_000_000n;
    const assetId = 1;

    beforeAll(async () => {
      // Sibling Paras
      const siblingParas = [1000, 3333];
      const siblingParaSovereignAccounts = siblingParas.map((paraId) =>
        sovereignAccountOfSibling(context, paraId)
      );

      // Fund all accounts
      const fundAmount = 100_000_000_000_000_000_000_000n;
      for (const address of siblingParaSovereignAccounts) {
        await fundAccount(address as `0x${string}`, fundAmount, context);
      }
    });

    it({
      id: "T01",
      title: "SiblingPara should be able to create and manage a foreign asset via XCM",
      test: async function () {
        const assetLocation = {
          parents: 1,
          interior: {
            X3: [{ Parachain: 1000 }, { PalletInstance: 1 }, { GeneralIndex: 1 }],
          },
        };

        const createForeignAssetCall = context
          .polkadotJs()
          .tx.evmForeignAssets.createForeignAsset(assetId, assetLocation, 18, "TEST", "TEST");

        const { blockRes: block1 } = await sendCallAsPara(
          createForeignAssetCall,
          1000,
          context,
          fundAmount / 20n
        );

        await expectSystemEvent(
          block1.block.hash,
          "evmForeignAssets",
          "ForeignAssetCreated",
          context
        );

        const createdForeignAsset = (
          await context.polkadotJs().query.evmForeignAssets.assetsById(assetId)
        ).toJSON();
        expect(createdForeignAsset).to.exist;
        expect(createdForeignAsset!["parents"]).to.eq(1);
        expect(createdForeignAsset!["interior"]["x3"][0]["parachain"]).to.eq(1000);
        expect(createdForeignAsset!["interior"]["x3"][1]["palletInstance"]).to.eq(1);
        expect(createdForeignAsset!["interior"]["x3"][2]["generalIndex"]).to.eq(1);

        const freezeCall = context
          .polkadotJs()
          .tx.evmForeignAssets.freezeForeignAsset(assetId, false);

        const { blockRes: block2 } = await sendCallAsPara(
          freezeCall,
          1000,
          context,
          fundAmount / 20n
        );
        await expectSystemEvent(
          block2.block.hash,
          "evmForeignAssets",
          "ForeignAssetFrozen",
          context
        );

        const unfreezeCall = context.polkadotJs().tx.evmForeignAssets.unfreezeForeignAsset(assetId);

        const { blockRes: block3 } = await sendCallAsPara(
          unfreezeCall,
          1000,
          context,
          fundAmount / 20n
        );
        await expectSystemEvent(
          block3.block.hash,
          "evmForeignAssets",
          "ForeignAssetUnfrozen",
          context
        );

        const newAssetLocation = {
          parents: 1,
          interior: {
            X3: [{ Parachain: 1000 }, { PalletInstance: 2 }, { GeneralIndex: 2 }],
          },
        };

        const changeLocationCall = context
          .polkadotJs()
          .tx.evmForeignAssets.changeXcmLocation(assetId, newAssetLocation);

        const { blockRes: block4 } = await sendCallAsPara(
          changeLocationCall,
          1000,
          context,
          fundAmount / 20n
        );
        await expectSystemEvent(
          block4.block.hash,
          "evmForeignAssets",
          "ForeignAssetXcmLocationChanged",
          context
        );

        const modifiedForeignAsset = (
          await context.polkadotJs().query.evmForeignAssets.assetsById(assetId)
        ).toJSON();
        expect(modifiedForeignAsset).to.exist;
        expect(modifiedForeignAsset!["parents"]).to.eq(1);
        expect(modifiedForeignAsset!["interior"]["x3"][0]["parachain"]).to.eq(1000);
        expect(modifiedForeignAsset!["interior"]["x3"][1]["palletInstance"]).to.eq(2);
        expect(modifiedForeignAsset!["interior"]["x3"][2]["generalIndex"]).to.eq(2);
      },
    });
  },
});
