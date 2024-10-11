import { createContext, useContext, useState } from 'react';

// Create a context for the user profile
const UserContext = createContext();

export const UserProvider = ({children}) => {
    const [user, setUser] = useState({
        name: 'John Doe',
        email: 'john.doe@example.com',
        age: 30,
        city: 'New York',
    });

   const updateUser = (updatedUser) => {
        setUser((prevUser) => ({
          ...prevUser,
          ...updatedUser,
        }));
   };

   return (
       <UserContext.Provider value={{ user, updateUser }}>
           {children}
       </UserContext.Provider>
   )
}

export const useUserContext = () => {
    return useContext(UserContext);
}

export default UserContext;


const { data: authUser } = useQuery({
    queryKey: ["authUser"],
    queryFn: async () => {
      try {
        const response = await fetchAuthUser();
        if (response.error) return null;
        // if (!response.ok) {
        //   throw new Error(response.error || "Failed to get user");
        // }
        console.log("authUser is here : ", response.data);
        return response.data;
      } catch (error) {
        console.log(error);
        throw new Error(error);
      }
    },
    retry: false,
  });