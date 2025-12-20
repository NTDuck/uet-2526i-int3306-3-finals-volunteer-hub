export const isLoggedIn = async () => {
  const response = await fetch("http://localhost:4000/api/me", {
    method: "GET",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
  });

  const body = await response.json()
  
  if (response.status === 200 && body.logged_in === true) {
    return true;
  }

  return false;
};

export const getRole = async () => {
  const response = await fetch("http://localhost:4000/api/me", {
    method: "GET",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
  });
  
  const body = await response.json()
  
  if (response.status === 200) {
    return body.role
  }

  return '';
};
