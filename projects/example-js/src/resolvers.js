const axios = require('axios');

const BASE_URL = 'http://localhost:3000'; // This is your upstream REST API

const resolvers = {
  Query: {
    // Resolver for fetching all posts
    posts: async () => {
      const response = await axios.get(`${BASE_URL}/posts`);
      return response.data;
    },

    // Resolver for fetching a single post by ID
    post: async (_, { id }) => {
      const response = await axios.get(`${BASE_URL}/posts/${id}`);
      return response.data;
    },

    // Resolver for fetching all users
    users: async () => {
      const response = await axios.get(`${BASE_URL}/users`);
      return response.data;
    },

    // Resolver for fetching a single user by ID
    user: async (_, { id }) => {
      const response = await axios.get(`${BASE_URL}/users/${id}`);
      return response.data;
    }
  },

  Post: {
    // Resolver for fetching the user of a post
    user: async (parent) => {
      const response = await axios.get(`${BASE_URL}/users/${parent.userId}`);
      return response.data;
    }
  }
};

module.exports = resolvers;
