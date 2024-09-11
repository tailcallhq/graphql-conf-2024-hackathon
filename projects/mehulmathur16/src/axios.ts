import axios from 'axios';
const qs = require('qs');

const axiosInstance = axios.create({
    baseURL: 'http://localhost:3000',
    timeout: 1000,
    paramsSerializer: params => {
        return qs.stringify(params, { arrayFormat: 'repeat' });
    }
});

export default axiosInstance;
