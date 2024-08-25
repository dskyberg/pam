
import { gql } from "@apollo/client";

export const GET_FULL_MATRIX = gql`
  query GetFullMatrix {
    matrix {
      categories {
        id
        name
        products {
          id
          name
          availability {
            id
            jurisdiction {
              id
            }
            lifecycle {
              id
              name
            }
            compliance {
              id
              name
            }
            lastUpdated
            comments {
              id
              text
              created
            }
          }
          features {
            id
            name
            availability {
              id
              jurisdiction {
                id
              }
              lifecycle {
                id
                name
              }
              compliance {
                id
                name
              }
              lastUpdated
              comments {
                id
                text
                created
              }
            }
          }
        }
      }
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
          csp
          cspRegion
          country
          region
        }
      }
    }
  }
`;

