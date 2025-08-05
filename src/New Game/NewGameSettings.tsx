import { TextField } from "@mui/material";
import { useState } from "react";
import { Button } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";

interface RegisterResult {
  success: boolean;
  message?: string
}

export default function NewGameSettings() {
  const [gameName, setGameName] = useState<string>("");
  const [companyName, setCompanyName] = useState<string>("");
  const [result, setResult] = useState<RegisterResult | null>(null);

  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log("Wow, your game's name is " + gameName);
    console.log("Sending game_name: " + gameName + ", company name: " + companyName);
    invoke<RegisterResult>('register', { game_name: gameName, company_name: companyName }).then((res: RegisterResult) => {
      setResult(res);
      if (res.success) {
        navigate("../home")
      }
    })
  };

  const handleGameNameInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    setGameName(e.target.value);
  }

  const handleCompanyNameInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCompanyName(e.target.value);
  }

  const renderResult = (res: RegisterResult | null) => {
    if (!res) return null;
    if (res.success) return <p
    style={{
      color: "limegreen"
    }}
    >
      {res.message || "Success"}
    </p>;
    return <p style={{color: "red"}}>
      {res.message || "An error occured"}
    </p>
  }

  return (
    <div className="new-game-settings">
      <h1>New game</h1>
      <form onSubmit={handleSubmit} autoComplete="off">
        <div className="new-game-form">
          <h2>Generate new SimIndustries World</h2>
          <div className="line">
            <h3>Game name: </h3>
            <TextField
              value={gameName}
              onChange={handleGameNameInput}
              id="gameName"
              className="text-new-game"
              label="Game Name"
              variant="outlined"
              sx={{
                label: {
                  color: "white",
                },
                "& .MuiOutlinedInput-notchedOutline": {
                  borderColor: "white",
                },
                "&:hover .MuiOutlinedInput-notchedOutline": {
                  borderColor: "white",
                },
              }}
            />
          </div>
          <div className="line">
            <h3>Company name: </h3> 
            <TextField 
            value={companyName}
            onChange={handleCompanyNameInput}
            label="Company name"
            />
          </div>
          <Button variant="contained" type="submit">
            Start Game
          </Button>
          {renderResult(result)}
        </div>
        
      </form>
      
    </div>
  );
}
