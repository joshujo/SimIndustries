import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import './home.css';

type PlayerData = {
  companyName: string;
  money: string;
  numberOfAssets: number;
};

export default function PlayerData() {
  const [data, setData] = useState<PlayerData | null>(null);

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;

    listen<PlayerData>("updatePlayerCompanyData", (event) => {
      setData(event.payload);
    }).then((unlistenFn) => {
      unlisten = unlistenFn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  return <div className="player-data-container">
    {
        data ? (
            <div>
                <p><strong>Company Name: </strong>{data.companyName}</p>
                <p><strong>Money: </strong>{data.money}</p>
                <p><strong>Number of Assets: </strong>{data.numberOfAssets}</p>
            </div>
        ) : (
            <p>
                "Loading player data"
            </p>
        )
    }
  </div>;
}
