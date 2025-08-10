import { Button, Modal } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";

type Asset = {
  type: string;
  value: string;
  production?: null | string;
  hasInventory: boolean;
  id: number;
};

export default function PlayerAssets() {
  let [assetData, setAssetData] = useState<Asset[] | null>(null);
  let unlisten = useRef<UnlistenFn | null>(null);

  useEffect(() => {
    listen<Asset[]>("playerAssetData", (event) => {
      setAssetData(event.payload);
    }).then((unlistenFn) => {
      unlisten.current = unlistenFn;
    });
    invoke("retrieve_player_assets");

    return () => {
      if (unlisten.current) {
        unlisten.current();
        unlisten.current = null;
      }
      invoke("unsub_retrieve_player_assets");
    };
  }, []);

  return (
    <div className="asset-container">
      <h1>Player Assets</h1>
      {assetData ? (
        <div className="asset-card-container">
          {assetData.map((data, index) => (
            <AssetCard key={index} {...data} />
          ))}
        </div>
      ) : (
        <div></div>
      )}
    </div>
  );
}

function AssetCard(props: Asset) {
  return (
    <div className="asset-card">
      <p>Type: {props.type}</p>
      <p>Value: ${props.value}</p>
      {props.production ? <p>Production: {props.production}</p> : <></>}
      {props.hasInventory && <InventoryModel id={props.id} />}
    </div>
  );
}

type Inventory = {
  good: string;
  amount: number;
};

function InventoryModel(props: { id: number }) {
  const { id } = props;
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const unlisten = useRef<UnlistenFn | null>();
  let [inventory, setInventory] = useState<Inventory[] | null>(null);

  useEffect(() => {
    listen<Inventory[]>("inventoryData" + id, (event) => {
      setInventory(event.payload);
    }).then((unlistenFn) => {
      unlisten.current = unlistenFn;
    });
    invoke("retrieve_inventory_data", { id: id });

    return () => {
      if (unlisten.current) {
        unlisten.current();
        unlisten.current = null;
      }
      invoke("unsub_retrieve_inventory_data", { id: id });
    };
  }, [id]);

  const renderInventory = () =>
    inventory?.map((data, index) => (
      <div key={index}>
        <p>
          <strong>Good: </strong>
          {data.good}
        </p>
        <p>
          <strong>Amount: </strong>
          {data.amount}
        </p>
      </div>
    ));

  return (
    <div>
      <Button onClick={handleOpen}>Inventory</Button>
      <Modal
        open={open}
        onClose={handleClose}
        aria-labelledby="Inventory Title"
      >
        <div className="inventory-modal">{renderInventory()}</div>
      </Modal>
    </div>
  );
}
