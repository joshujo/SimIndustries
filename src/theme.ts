import { createTheme } from "@mui/material/styles";

export const theme = createTheme({
    palette: {
        mode: "dark",
        primary: {
            main: "#2b2d31"
        },
        background: {
            default: "#2b2d31",
            paper: "#2b2d31"
        },
        text: {
            primary: "#ffffff"
        }
    },
    components: {
        MuiButton: {
            styleOverrides: {
                root: {
                    color: "#ffffff",
                    backgroundColor: "#191a1bff",
                    "&:hover": {
                        backgroundColor: "#1c168fff"
                    },
                    "&.Mui-expanded": {
                        color: "white"
                    },
                    "&:active": {
                        color: "white"
                    }
                }
            }
        },
        MuiTextField: {
            styleOverrides: {
                root: {
                    color: "white",
                    backgroundColor: "#323232ff",
                    "&:active": {
                        color: "#ffffff"
                    }
                }
            }
        },
        MuiInputLabel: {
            styleOverrides: {
                root: {
                    color: "#ffffff",
                    "&.Mui-focused": {
                        color: "#ffffff"
                    }
                }
            }
        }
    }
})

