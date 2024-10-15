// productPage/context/UsageContext.jsx
import React, { createContext, useState, useContext, useEffect } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import useUpdateUserProfile from "../api-service/ApiRequest";

// Create the context
const UsageContext = createContext();

// Custom hook to use the UsageContext
export const useUsage = () => useContext(UsageContext);

// Provider component
export const UsageProvider = ({ children }) => {
  const queryClient = useQueryClient();
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });
  console.log(authUser);
  const { updateProfile } = useUpdateUserProfile();

  //   const initialUsageCount =
  //     authUser && authUser.usage?.tries_used ? authUser.usage.tries_used : 0;

  //   const maxUsage = authUser ? authUser.usage?.max_tries || 10 : 5;

  //   const [usageCount, setUsageCount] = useState(initialUsageCount);

  // Fetch usageCount from localStorage for non-authenticated users
  const getLocalStorageUsageCount = () => {
    const storedUsage = localStorage.getItem("usageCount");
    return storedUsage ? parseInt(storedUsage, 10) : 0;
  };

  const initialUsageCount = authUser
    ? authUser.usage?.tries_used || 0
    : getLocalStorageUsageCount(); // Use localStorage for non-auth users

  const maxUsage = authUser ? authUser.usage?.max_tries || 10 : 5;

  const [usageCount, setUsageCount] = useState(initialUsageCount);

  // Sync usageCount to localStorage for non-authenticated users
  useEffect(() => {
    if (!authUser) {
      localStorage.setItem('usageCount', usageCount);
    }
    queryClient.fetchQuery({ queryKey: ["authUser"] });

  }, [usageCount, authUser]);

  // Handler to update the usage count
  const incrementUsage = () => {
    if (usageCount < maxUsage) {
      setUsageCount((prev) => prev + 1);
      if (authUser) {
        authUser.usage.tries_used += 1; // Update client-side authUser tries
        updateProfile({
          tries_used: authUser.usage.tries_used,
        });
      } else {
        localStorage.setItem('usageCount', usageCount + 1); // Update localStorage for non-auth users
      }
    }
  };

  // Disable translate button when the limit is reached
  const isLimitReached = usageCount >= maxUsage;

  return (
    <UsageContext.Provider
      value={{ usageCount, maxUsage, incrementUsage, isLimitReached }}
    >
      {children}
    </UsageContext.Provider>
  );
};
