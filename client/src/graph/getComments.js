import { gql } from "@apollo/client"

export const GET_COMMENTS = gql`
  query GetComments($itemId: String!) {
    comments(itemId: $itemId) {
        id
        text
        createdBy
        created
    }
}
`