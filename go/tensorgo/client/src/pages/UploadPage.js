import "./App.css";
import axios from "axios";
import { useState } from "react";
import { Link } from "react-router-dom";
const BACKEND_HOST = process.env.BACKEND_HOST || "localhost";
const UploadPage = () => {
  const [file, setFile] = useState();
  const handleSubmit = async (event) => {
    event.preventDefault();
    const formData = new FormData();
    formData.append("image", file);
    try {
      const response = await axios.post(
        `http://${BACKEND_HOST}:12345/upload`,
        formData,
        {
          headers: {},
        }
      );
      console.log(response);
    } catch (error) {
      console.log(error);
    }
  };
  const handleFileChange = (event) => {
    setFile(event.target.files[0]);
  };
  return (
    // <div className="App"style={{ display: "flex", flexDirection: "column", alignItems: "center"}}>
    <div className="App">
      <Link to={"/"}>
        <button className="home-button">home</button>
      </Link>
      <Link to={"/search"}>
        <button className="home-button">search</button>
      </Link>
      <div className="container">
        <h1 className="image-upload">Upload your tensorGO images</h1>
        <p className="image-description">Choose your files and press submit</p>
        <div>
          <input
            type="file"
            id="actual-btn"
            hidden
            onChange={handleFileChange}
          />
          <label className="file-label" for="actual-btn">
            {!file ? "choose file" : "change file"}
          </label>
          <span id="file-chosen">{file?.name}</span>
        </div>
        <button className="submit-button" onClick={handleSubmit}>
          submit
        </button>
      </div>
    </div>
  );
};
export default UploadPage;
