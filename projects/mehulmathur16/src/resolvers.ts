import axiosInstance from './axios';
import { IContext } from './types';

const resolvers = {
    Query: {
        posts: async (_: any, __: any, { cache }: IContext) => {
            if (cache.postsList) {
                return cache.postsList;
            }
            const response = await axiosInstance.get('/posts');
            cache.postsList = response.data;

            response.data.map((currPost: any) => {
                cache.postMap[currPost.id] = currPost;
            })

            return response.data;
        },
        post: async (_: any, { id }: { id: number }, { cache, postDataLoader }: IContext) => {
            if (cache.postMap[id]) {
                return cache.postMap[id];
            }
            return postDataLoader.load(id);
        },
        users: async (_: any, __: any, { cache }: IContext) => {
            if (cache.usersList) {
                return cache.usersList;
            }
            const response = await axiosInstance.get('/users');
            cache.usersList = response.data;

            response.data.map((currUser: any) => {
                cache.userMap[currUser.id] = currUser;
            })

            return response.data;
        },
        user: async (_: any, { id }: { id: number }, { cache, userDataLoader }: IContext) => {
            if (cache.userMap[id]) {
                return cache.userMap[id];
            }
            return userDataLoader.load(id);
        }
    },
    Post: {
        user: async (post: { userId: number }, _: any, { cache, userDataLoader }: IContext) => {
            if (cache.userMap[post.userId]) {
                return cache.userMap[post.userId]
            }
            const res = userDataLoader.load(post.userId)
            return res;
        }
    },
};

export default resolvers;
