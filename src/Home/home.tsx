import PlayerData from "./PlayerData";
import DateTimePanel from "./DateTimePanel";
import PlayerAssets from "./PlayerAssets";
import BuyAssets from "./BuyAssets";

export default function Home() {

    return (
        <div>
            <PlayerData/>
            <DateTimePanel/>
            <PlayerAssets />
            <BuyAssets />
        </div>
    )
}