import { AxiosError } from 'axios';

export function mapCatchError(error: unknown | any, def?: string): string {
  if (error instanceof AxiosError) {
    if (error.response?.data?.message) {
      return error.response.data.message;
    }
  }
  if (error instanceof Error) {
    return error.message;
  }
  return def || 'An error occurred';
}
