import { gql } from '@apollo/client';


export const GET_MATRIX = gql`
  query GetMatrix {
    matrix {
      compliances {
        id
        name
      }
      lifecycles {
        id
        name
        description
      }
      jurisdictions {
        id
        name
        title
        cells {
            id
            name
        }
    }
  }
}
`