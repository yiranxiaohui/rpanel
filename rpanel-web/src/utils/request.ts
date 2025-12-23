import axios from "axios";

const instance = axios.create({
  baseURL: 'https://some-domain.com/api/',
  timeout: 1000,
  headers: {'X-Custom-Header': 'foobar'}
});

instance.interceptors.response.use(undefined, async (error) => {
  if (error.response?.status === 401) {
    return instance(error.config); // 重新发送原始请求
  }
  throw error;
});