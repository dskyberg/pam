import { gql } from "@apollo/client";

export const UPDATE_AVAILABILITY = gql`
mutation UpdateAvailability($input: AvailabilityUpdateInput!) {
  updateAvailability(input:$input) {
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
`