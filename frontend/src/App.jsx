import { Navigate, Route, Routes } from "react-router-dom";
import MarketingPage from "./marketing-page/MarketingPage";
import SignInSide from "./sign-in-side/SignInSide";
import Blog from "./marketing-page/Blog";
import ProductPage from "./product-page/ProductPage";
import { useQuery } from "@tanstack/react-query";
import { fetchAuthUser } from "./api-service/ApiRequest";

function App() {
  const { data: authUser, isLoading } = useQuery({
    queryKey: ["authUser"],
    queryFn: async () => {
      try {
        const data = await fetchAuthUser();
        if (data.error) return null;
        if (!response.ok) {
          throw new Error(data.error || "Failed to get user");
        }
        console.log("authUser is here : ", data);
        return data;
      } catch (error) {
        console.log(error);
        throw new Error(error);
      }
    },
    retry: false,
  });
  return (
    <Routes>
      <Route
        path="/"
        element={<MarketingPage />}
      />
      <Route
        path="/login"
        element={!authUser ? <SignInSide /> : <Navigate to="/" />}
      />
      <Route path="/blog" element={<Blog />} />
      <Route path="/product" element={<ProductPage />} />
    </Routes>
  );
}

export default App;
