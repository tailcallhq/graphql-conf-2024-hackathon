import DataLoader from 'dataloader';
import http from 'http';
import axiosInstance from './axios';

const httpAgent = new http.Agent({ keepAlive: true });

export const userLoader = new DataLoader<number, any>(async (ids) => {
    const response = await axiosInstance.get('/users', {
        params: { id: ids },
        httpAgent
    });
    const users = response.data;
    const userMap = new Map(users.map((user: any) => [user.id, user]));
    return ids.map(id => userMap.get(id));
});


