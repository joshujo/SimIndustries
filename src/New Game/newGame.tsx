import "./newGame.css";
import Button from "@mui/material/button";
import { useNavigate } from "react-router";

export default function NewGame() {
    const navigate = useNavigate();

    const toStart = ()  => {
        navigate("/new_game_settings")
    }


  return (
    <div className="new-game-intro">
      <h1>SimIndustries</h1>
        <div className="new-game-container">
          <h2>New Game</h2>
          <Button variant="contained"
          onClick={() => {
            toStart()
          }}
          >Start Game</Button>
        </div>
    </div>
  );
}

