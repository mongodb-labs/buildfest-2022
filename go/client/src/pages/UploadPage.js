import axios from "axios";
import { useState } from "react";

const BACKEND_HOST = process.env.BACKEND_HOST || "localhost";

const UploadPage = () => {
  const [file, setFile] = useState();

  const handleSubmit = async (event) => {
    event.preventDefault();
    const formData = new FormData();
    formData.append("file", file);
    try {
      const response = await axios.post(BACKEND_HOST, formData, {
        headers: {},
      });
      console.log(response);
    } catch (error) {
      console.log(error);
    }
  };

  const handleFileChange = (event) => {
    setFile(event.target.files[0]);
  };

  return (
    <div
      className="App"
      style={{ display: "flex", flexDirection: "column", alignItems: "center" }}
    >
      <h1>Upload Images</h1>
      <h2>This page allows you to upload an image to the app.</h2>
      <input type="file" onChange={handleFileChange} />
      <button onClick={handleSubmit}>Submit</button>
    </div>
  );
};

export default UploadPage;
