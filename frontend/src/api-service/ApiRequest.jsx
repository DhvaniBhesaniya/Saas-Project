// import Cookies from "js-cookie";

// // ApiRequest.jsx

// // Helper function to get token from localStorage and set headers
// const getHeaders = () => {
//   const token = Cookies.get("token");
//   console.log(token);
//   return {
//     "Content-Type": "application/json",
//     ...(token ? { token: `${token}` } : {}),
//   };
// };

// console.log(getHeaders())

// Function to handle user login
export const loginUser = async (email, password) => {
  try {
    const response = await fetch(`/api/user/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },

      body: JSON.stringify({ email, password }),
    });

    const responseData = await response.json();

    // // Assuming the token comes in the response and you want to save it in localStorage
    // const { token } = responseData;
    // if (token) {
    //   Cookies.set("token", token, { expires: 1 });
    // }

    return responseData; // Return the response data for further use
  } catch (error) {
    console.error("Error logging in:", error);
    throw error;
  }
};

// Function to handle user registration
export const registerUser = async (name, email, password) => {
  try {
    const response = await fetch(`/api/user/register`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, email, password }),
    });

    const responseData = await response.json();

    return responseData; // Return the response data for further use
  } catch (error) {
    console.error("Error registering:", error);
    throw error;
  }
};

// Function to fetch authenticated user data
export const fetchAuthUser = async () => {
  try {
    const response = await fetch(`/api/user/userdata`, {
      method: "GET",
     headers: { "Content-Type": "application/json" },
    });

    const responseData = await response.json();

    if (!response.ok) {
      throw new Error(responseData.message || "Failed to get user..");
    }

    return responseData; // Return the user data
  } catch (error) {
    console.error("Error fetching authenticated user:", error);
    throw error;
  }
};
