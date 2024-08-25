import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

export default function Home() {

    return (
        <Box display="flex" alignItems="center" justifyContent="center" sx={{
            width: "100%", margin: "1em"
        }}>
            <Typography variant="h4" component="h1" sx={{ mb: 2 }} align="center">
                Product Availability Matrix
            </Typography>
        </Box>
    )
}
