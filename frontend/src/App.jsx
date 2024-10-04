import { Navigate, Route, Routes } from "react-router-dom";
import MarketingPage from "./marketing-page/MarketingPage";
import SignInSide from "./sign-in-side/SignInSide";
import Blog from "./marketing-page/Blog";
import ProductPage from "./product-page/ProductPage";
import { useQuery } from "@tanstack/react-query";
import { fetchAuthUser } from "./api-service/ApiRequest";
import { Toaster } from "react-hot-toast";

function App() {
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
  return (
    <>
    <Routes>
      <Route path="/" element={<MarketingPage />} />
      <Route
        path="/login"
        element={!authUser ? <SignInSide /> : <Navigate to="/" />}
      />
      <Route path="/blog" element={<Blog />} />
      <Route path="/product" element={<ProductPage />} />
    </Routes>
    <Toaster />
    </>
  );
}

export default App;
