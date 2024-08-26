import { useQuery, useMutation } from '@apollo/client';
import { GET_FEATURE } from '../graph';

import { CircularProgress, Stack, Typography } from "@mui/material"

import Comments from "./Comments";
import Loading from "./Loading"

export function FeaturePanel({ item }) {
    const { data, loading, error } = useQuery(GET_FEATURE, { variables: { id: item.id } });

    if (loading) return <Loading />;
    if (error) {
        console.log("FeaturePanel error:", error.message)
    }
    let feature = data.feature;
    return (
        <Stack sx={{ paddingLeft: "1em", paddingRight: "1em" }}>
            <Typography variant="h6">Feature</Typography>
            <Typography >{feature.name}</Typography>
            <Typography >{feature.id}</Typography>

            <Comments itemId={feature.id} />
        </Stack>
    )
}

