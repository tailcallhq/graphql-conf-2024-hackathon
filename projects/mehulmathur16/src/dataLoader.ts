import DataLoader from 'dataloader';
import axiosInstance from './axios';

export const userLoader = new DataLoader<number, any>(async (ids) => {
    const response = await axiosInstance.get('/users', {
        params: { id: ids },
    });
    const users = response.data;
    const userMap = new Map(users.map((user: any) => [user.id, user]));
    return ids.map(id => userMap.get(id));
});


