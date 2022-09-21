const ROUTE_BASE = 'http://localhost:8080';

export async function PostData(route: string, payload: object) {
  const result = await fetch(`${ROUTE_BASE}${route}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  })
    .then((response) => {
      return response.json();
    })
    .then((data) => {
      console.log('posted!');
      return true;
    })
    .catch((error) => {
      console.log(error);
      return false;
    });
  return result;
}

export async function GetData(route: string) {
  const uri = `${ROUTE_BASE}${route}`;
  console.log(uri);
  const result = await fetch(uri, {
    method: 'GET',
  })
    .then((response) => response.json())
    .then((data) => {
      console.log(data);
      return data;
    })
    .catch((error) => {
      console.log(error);
      return false;
    });
  return result;
}
