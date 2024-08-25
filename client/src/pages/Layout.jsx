import { Outlet } from "react-router-dom";

import Container from "@mui/material/Container";
import Toolbar from "@mui/material/Toolbar";

export default function Layout() {
    return (
        <div sx={{ width: "100%", height: "100%", margin: 0, padding: 0 }}>
            <Toolbar />
            <Outlet sx={{ width: "100%", padding: 0, margin: 0 }} />
        </div>
    )
}