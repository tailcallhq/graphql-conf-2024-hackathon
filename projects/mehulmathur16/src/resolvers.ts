import axiosInstance from './axios';

const resolvers = {
    Query: {
        posts: async () => {
            const response = await axiosInstance.get('/posts');
            return response.data;
        },
        post: async (_: any, { id }: { id: number }) => {
            const response = await axiosInstance.get(`/posts/${id}`);
            return response.data;
        },
        users: async () => {
            const response = await axiosInstance.get('/users');
            return response.data;
        },
        user: async (_: any, { id }: { id: number }, context: any) => {
            return context?.userDataLoader.load(id);
        }
    },
    Post: {
        user: async (post: { userId: number }, args: any, context: any) => {
            const res = context?.userDataLoader.load(post.userId)
            return res;
        }
    },
    User: {
        posts: async (user: { id: number }) => {
            const response = await axiosInstance.get('/posts', {
                params: { userId: user.id }
            });
            return response.data;
        }
    }
};

export default resolvers;
