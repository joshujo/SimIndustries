import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";

type DateTime = {
    day: string,
    month: string,
    year: string,
    hour: string,
    minute: string,
    second: string
}

export default function DateTimePanel() {
    const [data, setData] = useState<DateTime | null>(null);
    let unlisten = useRef<UnlistenFn | null>(null);

    useEffect(() => {
        

        listen<DateTime>("timeAndDate", (event) => {
            setData(event.payload);
        }).then((unlistenFn) => {
            unlisten.current = unlistenFn;
        });

        return () => {
            if (unlisten.current) {
                unlisten.current();
                unlisten.current = null;
            };
        }
    }, []);

    return (
        <div className="date-time-panel">
            {
                data ? (
                    <div>
                        <p><strong>Date: </strong>{data.day}/{data.month}/{data.year}</p>
                        <p><strong>Time: </strong>{data.hour}:{data.minute}:{data.second}</p>
                    </div>
                ) : (
                    <p>
                        Loading time...
                    </p>
                )
            }
        </div>
    )
}