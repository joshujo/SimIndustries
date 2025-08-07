import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";

type Asset = {
  type: string;
  value: string;
  production?: null | string;
  numberOfItems?: null | number;
};


export default function PlayerAssets() {
  let [assetData, setAssetData] = useState<Asset[] | null>(null);
  let unlisten = useRef<UnlistenFn | null>(null);

  useEffect(() => {
    invoke("retrieve_player_assets");
    listen<Asset[]>("playerAssetData", (event) => {
        setAssetData(event.payload)
    }).then((unlistenFn) => {
        unlisten.current = unlistenFn;
    });

    return () => {
    if (unlisten.current) {
        unlisten.current();
        unlisten.current = null;
    };
      invoke("unsub_retrieve_player_assets");
    };
  }, []);

  return <div className="asset-container">
    <h1>Player Assets</h1>
    {
        assetData ? (
            <div>
                {
                    assetData.map((data, index) => (
                        
                            <AssetCard key={index} {...data}/>
                        
                    ))
                }
            </div>
         ) : (
            <div></div>
         )
    }
  </div>;
}

function AssetCard(props: Asset) {
  return (
    <div className="asset-card">
      <p>Type: {props.type}</p>
      <p>Value: {props.value}</p>
    </div>
  );
}
