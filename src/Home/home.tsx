import PlayerData from "./PlayerData";
import DateTimePanel from "./DateTimePanel";
import PlayerAssets from "./PlayerAssets";

export default function Home() {

    return (
        <div>
            <PlayerData/>
            <DateTimePanel/>
            <PlayerAssets />
        </div>
    )
}