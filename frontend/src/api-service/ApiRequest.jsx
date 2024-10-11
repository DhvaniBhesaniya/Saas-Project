// import Cookies from "js-cookie";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import toast from "react-hot-toast";


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

 const useUpdateUserProfile = () => {
  const queryClient = useQueryClient();

  const { mutateAsync: updateProfile, isPending: isUpdatingProfile, error: updateError } =
    useMutation({
      mutationFn: async (formData) => {
        try {
          console.log(formData);
          const res = await fetch(`/api/user/updateuser`, {
            method: "POST",
            headers: {
              "Content-Type": " application/json",
            },
            body: JSON.stringify(formData),
          });
          const data = await res.json();
          if (!res.ok) {
            throw new Error(data.error || "Something went wrong");
          }
          return data;
        } catch (error) {
          throw new Error(error.message);
        }
      },
      onSuccess: () => {
        toast.success(" Updated successfully");
        Promise.all([
          queryClient.invalidateQueries({ queryKey: ["authUser"] }),
        ]);
      },
      onError: (error) => {
        toast.error(error.message);
      },
    });

  return { updateProfile, isUpdatingProfile ,updateError};
};

 export default useUpdateUserProfile;



// Gen ai api request


export const getGenaiTranslatedText = async (textformData) => {
  try {
    const response = await fetch(`/api/genai/text`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },

      body: JSON.stringify(textformData),
    });

    const responseData = await response.json();

    if (!response.ok) {
      throw new Error(responseData.message || "Failed to get text..");
    }

    return responseData; // Return the user data
  } catch (error) {
    console.error("Error fetching Translated text:", error);
    throw error;
  }
};
export const getGenaiChat = async (ChatformData) => {
  try {
    const response = await fetch(`/api/genai/chat`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },

      body: JSON.stringify(ChatformData),
    });

    const responseData = await response.json();

    if (!response.ok) {
      throw new Error(responseData.message || "Failed to ai response..");
    }

    return responseData; // Return the user data
  } catch (error) {
    console.error("Error fetching ai response:", error);
    throw error;
  }
};
export const getGenaiDoc = async (docFormData) => {
  const response = await fetch("/api/genai/doc", {
    method: "POST",
    body: docFormData,
  });

  // Check if the response is ok, if not, throw an error
  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.message || "Failed to translate document.");
  }
  // console.log(response)

  // If successful, return the blob and headers (for file name)
  const blob = await response.blob();
  const contentDisposition = response.headers.get("Content-Disposition");
  const fileName = contentDisposition
    ? contentDisposition.split("filename=")[1].replace(/"/g, "")
    : "translated_document.txt"; // Fallback file name

  return { blob, fileName };
};

