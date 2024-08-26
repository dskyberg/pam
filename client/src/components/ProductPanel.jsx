import { useQuery, useMutation } from '@apollo/client';
import { GET_PRODUCT } from '../graph';

import { CircularProgress, Stack, Typography, Box } from "@mui/material"

import Comments from "./Comments";
import Loading from './Loading'

export function ProductPanel({ item }) {
    const { data, loading, error } = useQuery(GET_PRODUCT, { variables: { id: item.id } });

    if (loading) return <Loading />;
    if (error) {
        console.log("ProductPanel error:", error.message)
    }
    let product = data.product;
    return (
        <Stack sx={{ paddingLeft: "1em", paddingRight: "1em" }}>
            <Typography variant="h6">Product</Typography>
            <Typography >{product.name}</Typography>
            <Typography >{product.id}</Typography>

            <Comments itemId={product.id} />
        </Stack>
    )
}

