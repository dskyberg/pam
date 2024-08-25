import { version } from "react";
import { useNavigate } from "react-router-dom";
import packageJson from '../../package.json';
import { Button, Card, CardActions } from "@mui/material"
import { CardContent } from "@mui/material";
import { Typography } from "@mui/material";
import { grey } from '@mui/material/colors';


export default function About() {
    const navigate = useNavigate();

    return (

        <Card sx={{ my: 4, margin: "1em" }}>
            <CardContent>
                <Typography variant="h4" component="h1" sx={{ mb: 2 }}>
                    About Page
                </Typography>
                <Typography sx={{ mb: 2 }} >
                    {packageJson.description}
                </Typography>

                <Typography sx={{ mb: 2 }} >
                    Version {packageJson.version}
                </Typography>
                <Typography sx={{ mb: 2 }}>
                    React {version}
                </Typography>
            </CardContent>
            <CardActions sx={{ justifyContent: "center", backgroundColor: grey[50] }}>
                <Button color="inherit" onClick={() => navigate('/')}>
                    Return Home
                </Button>
            </CardActions>
        </Card>

    )
}
