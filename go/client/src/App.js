import React from "react";
import SearchPage from "./pages/SearchPage";
import UploadPage from "./pages/UploadPage";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/upload",
    element: <UploadPage />,
  },
  {
    path: "/search",
    element: <SearchPage />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
