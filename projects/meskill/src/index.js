const { ApolloServer } = require('apollo-server');
const typeDefs = require('./schema');
const resolvers = require('./resolvers');

// Initialize Apollo Server
const server = new ApolloServer({
  typeDefs,
  resolvers
});

// Start the server
server.listen({ port: 8000 }).then(({ url }) => {
  console.log(`🚀 Server ready at ${url}`);
});
