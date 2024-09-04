const { gql } = require('apollo-server');

const typeDefs = gql`
  type Query {
    posts: [Post]
    post(id: ID!): Post
    users: [User]
    user(id: ID!): User
  }

  type Post {
    id: ID
    title: String
    body: String
    user: User
  }

  type User {
    id: ID
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
