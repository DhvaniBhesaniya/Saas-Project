import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import MarketingPage from "./marketing-page/MarketingPage";
import SignInSide from "./sign-in-side/SignInSide";
import Blog from "./marketing-page/Blog";
import ProductPage from "./product-page/ProductPage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/home" element={<MarketingPage />} />
        <Route path="/login" element={<SignInSide />} />
        <Route path="/blog" element={<Blog />} />
        <Route path="/product" element={<ProductPage/>} />
        
      </Routes>
    </Router>
  );
}

export default App;
