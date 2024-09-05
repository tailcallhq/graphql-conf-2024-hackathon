const { gql } = require('apollo-server');

const typeDefs = gql`
  type Query {
    posts: [Post]
    post(id: Int!): Post
    users: [User]
    user(id: Int!): User
  }

  type Post {
    id: Int
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

module.exports = typeDefs;
