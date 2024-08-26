
import { useState, useEffect } from 'react';
import { useQuery, useMutation } from "@apollo/client";
import { GET_COMMENTS, CREATE_COMMENT } from '../graph';

import { styled } from '@mui/material/styles';

import { CircularProgress } from "@mui/material";
import { Stack, List, ListItem, ListItemText, Typography, FormControl, InputLabel, Input, InputAdornment, IconButton } from '@mui/material';
import CheckCircleTwoToneIcon from '@mui/icons-material/CheckCircleTwoTone';
import CancelTwoToneIcon from '@mui/icons-material/CancelTwoTone';



const LeftIconButton = styled(IconButton)(
    ({ theme }) => ({
        paddingRight: 0
    }),
);

const RightIconButton = styled(IconButton)(
    ({ theme }) => ({
        paddingLeft: 0
    }),
);

export default function Comments({ itemId }) {
    const [newComment, setNewComment] = useState('');
    useEffect(() => {
        setNewComment('')
    }, [itemId])


    const { loading, error, data } = useQuery(GET_COMMENTS, {
        variables: { itemId: itemId }
    });

    // addResult = {loading, error, data}
    const [addComment, addResult] = useMutation(CREATE_COMMENT,
        {
            refetchQueries: [
                GET_COMMENTS, // DocumentNode object parsed with gql
                'GetComments' // Query name
            ]
        },
    );

    if (loading) return <CircularProgress />;

    if (error) return (
        <pre>{JSON.stringify(error)}</pre>
    )

    const handleSaveComment = () => {
        console.log("Saving comment:", newComment)
        addComment({
            variables: {
                input: {
                    itemId,
                    text: newComment
                }
            }
        }).then(() => {
            setNewComment('');
        }).catch(err => console.log("Comment save error:", err.message))

    }
    const handleCancelComment = () => {
        setNewComment('')
    }

    return (
        <Stack sx={{ marginTop: "1em" }}>
            <Typography variant="h6">Comments</Typography>
            <List sx={{ bgcolor: 'background.paper' }}>
                {data.comments.map(comment => (
                    <ListItem key={comment.id}>
                        <ListItemText primary={comment.text} secondary={comment.created} />
                    </ListItem>
                ))}
            </List>
            <FormControl fullWidth sx={{ m: 1 }} variant="standard">
                <InputLabel htmlFor="new-comment">Add Comment</InputLabel>
                <Input
                    id="new-commentt"
                    value={newComment}
                    onChange={(event) => { setNewComment(event.target.value) }}
                    endAdornment={<InputAdornment position="end">
                        <>
                            <LeftIconButton
                                aria-label="Cancel"
                                onClick={handleCancelComment}
                            >
                                <CancelTwoToneIcon />
                            </LeftIconButton>
                            <RightIconButton
                                aria-label="Save"
                                onClick={handleSaveComment}
                            >
                                <CheckCircleTwoToneIcon />
                            </RightIconButton>
                        </>
                    </InputAdornment>}
                />
            </FormControl>
        </Stack >
    )
}