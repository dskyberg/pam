import { Box, CircularProgress } from "@mui/material"
export default function Loading(props) {
    return (
        <Box sx={{ display: "flex", width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
            <CircularProgress {...props} />
        </Box>
    )
}