# 🚀 GraphQL Conf Hackathon 2024 | Tailcall

## Objective

The idea is to implement the fastest GraphQL server with the following schema:

```graphql
schema {
  query: Query
}

type Query {
  posts: [Post]
  post(id: Int!): Post
  users: [User]
  user(id: Int!): User
}

type Post {
  id: Int
  userId: Int!
  title: String
  body: String
  user: User
}

type User {
  id: Int
  name: String
  username: String
  email: String
  address: Address
  phone: String
  website: String
  posts: [Post]
}

type Address {
  zipcode: String
  geo: Geo
}

type Geo {
  lat: Float
  lng: Float
}
```

## Technical Requirements

1. All CI tests should pass.
2. Your implementation should be under the `/projects` directory.

## Additional Requirements

1. Your implementation has to be the fastest amongst all the contributors.
2. Any kind of plagiarism will result in a ban, checkout our [guidelines](https://tailcall.run/docs/contributors/bounty/#identifying-plagiarism) on plagiarism for more.

## And Some More...

- We might add new tests and modify the existing ones to ensure its there is no hardcoding and its a level playing field for all.
- If you questions or doubts about the hackathon, connect with us on [Discord] or [Twitter] or the only two people in that bright yellow T-Shirt they'd be glad to say 👋.

### Rules

- The solution should be provided as pull-request to this repo from participant's fork.
- The pull-request should contain only file additions inside `/projects/${participant_name}` without changes in the other repo files or other participants code.
- The solution could be implemented in any language or framework or using specific tools within the scope of the license granted by used tools. The only prohibition is the use of the [tailcall](https://github.com/tailcallhq/tailcall/) tool.
- The solution should contain all source code and setup that is required to understand how the solution was achieved and how to run it.
- Cooperation on single solution is acceptable, but only the author of the pr will be eligible to win the prize.
- In case of the multiple solutions with identical code will be candidates for prize only the solution that was added first will be eligible for prize.
