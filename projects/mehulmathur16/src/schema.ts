const Schema = `
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
`;

export default Schema;