import { gql } from "@apollo/client";

export const CREATE_COMMENT = gql`
mutation CreateComment($input: CommentInput!) {
  createComment(input:$input) {
    id
    text
    createdBy
    created
  }
}
`