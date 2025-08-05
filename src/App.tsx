import "./App.css";
import { HashRouter as Router, Routes, Route } from "react-router-dom";
import NewGame from "./New Game/newGame";
import NewGameSettings from "./New Game/NewGameSettings";
import {theme} from "./theme";
import { ThemeProvider } from "@mui/material";
import Home from "./Home/home";

function App() {
  return (
    <ThemeProvider theme={theme}>
    <Router>
      <Routes>
        <Route path="/" element={<NewGame />} />
        <Route path="/new_game_settings" element={<NewGameSettings />} />
        <Route path="/home" element={<Home/>} />
      </Routes>
    </Router>
    </ThemeProvider>
  );
}

export default App;
