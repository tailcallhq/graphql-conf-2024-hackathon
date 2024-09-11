import axiosInstance from './axios';

const resolvers = {
    Query: {
        posts: async (_: any, __: any, { cache }: any) => {
            if (cache['posts']) {
                return cache['posts'];
            }
            const response = await axiosInstance.get('/posts');
            cache['posts'] = response.data;
            return response.data;
        },
        post: async (_: any, { id }: { id: number }, context: any) => {
            return context?.postDataLoader.load(id);
        },
        users: async (_: any, __: any, { cache }: any) => {
            if (cache['users']) {
                return cache['users'];
            }
            const response = await axiosInstance.get('/users');
            cache['users'] = response.data;
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
