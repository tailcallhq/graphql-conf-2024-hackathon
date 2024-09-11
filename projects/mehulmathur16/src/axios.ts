import axios from 'axios';
import http from 'http'
const qs = require('qs');

const httpAgent = new http.Agent({ keepAlive: true });

const axiosInstance = axios.create({
    baseURL: 'http://localhost:3000',
    timeout: 1000,
    paramsSerializer: params => {
        return qs.stringify(params, { arrayFormat: 'repeat' });
    },
    httpAgent,
});

export default axiosInstance;
