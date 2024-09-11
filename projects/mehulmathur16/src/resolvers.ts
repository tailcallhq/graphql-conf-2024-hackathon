import axios from 'axios';

const resolvers = {
    Query: {
        // Fetch all posts
        posts: async () => {
            const response = await axios.get('http://localhost:3000/posts');
            return response.data;
        },

        // Fetch a single post by ID
        post: async (_: any, { id }: { id: number }) => {
            const response = await axios.get(`http://localhost:3000/posts/${id}`);
            return response.data;
        },

        // Fetch all users
        users: async () => {
            const response = await axios.get('http://localhost:3000/users');
            return response.data;
        },

        // Fetch a single user by ID
        user: async (_: any, { id }: { id: number }) => {
            const response = await axios.get(`http://localhost:3000/users/${id}`);
            return response.data;
        }
    },

    // Resolvers for Post type
    Post: {
        user: async (post: { userId: number }) => {
            const response = await axios.get(`http://localhost:3000/users/${post.userId}`);
            return response.data;
        }
    },

    // Resolvers for User type
    User: {
        posts: async (user: { id: number }) => {
            const response = await axios.get(`http://localhost:3000/posts`, {
                params: { userId: user.id }
            });
            return response.data;
        }
    }
};

export default resolvers;
