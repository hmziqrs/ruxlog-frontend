import axios, { AxiosError } from 'axios';
import { camelizeKeys, decamelizeKeys } from 'humps';

export const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json',
    'csrf-token': process.env.NEXT_PUBLIC_CSRF_TOKEN,
    'Content-Encoding': 'gzip'
  }
});

api.interceptors.request.use((config) => {
  config.data = decamelizeKeys(config.data);
  return config;
});

api.interceptors.response.use(
  (response) => {
    return camelizeKeys(response.data) as any;
  }, // Return successful responses as-is
  (error: AxiosError) => {
    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      const { data } = error.response;

      if (typeof data === 'string') {
        try {
          // Try to parse the string as JSON
          const jsonData = JSON.parse(data);

          // Check if the parsed JSON has the expected format
          if (
            jsonData &&
            typeof jsonData === 'object' &&
            'message' in jsonData
          ) {
            // Replace the original error data with the parsed JSON
            error.response.data = jsonData;
          }
        } catch (parseError) {
          // If parsing fails, leave the original error as is
          console.warn('Error parsing response as JSON:', parseError);
        }
      }
    }

    // Return the error for further error handling
    return Promise.reject(error);
  }
);
