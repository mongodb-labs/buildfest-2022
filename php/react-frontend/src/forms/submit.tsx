import { PostData, GetData } from '../api';

export async function submit(route: string, payload: object) {
  console.log(await PostData(route, payload));
}
