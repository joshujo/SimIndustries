import { Modal } from "@mui/material";
import { useState } from "react";

export default function BuyAssets() {
    const [open, setOpen] = useState(false);
    const handleOpen = () => setOpen(true);
    const handleClose = () => setOpen(false);

    return (
        <div className="buy-assets">
            <button onClick={handleOpen}>Purchase Assets</button>
            <Modal open={open} onClose={handleClose}>
                <div className="buy-assets-model">

                </div>
            </Modal>
        </div>
    )
}