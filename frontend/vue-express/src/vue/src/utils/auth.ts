import { getCookie } from "./cookie_helper";

export const isLoggedIn = async () => {
  const response = await fetch("http://localhost:4000/api/me", {
    method: "GET",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
  });

  if (response.status === 200) {
    return true;
  }

  return false;
};

export const getRole = () => {
  return getCookie("user-role");
};
