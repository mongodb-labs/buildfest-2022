const UploadPage = () => {
  return (
    <div
      className="App"
      style={{ display: "flex", flexDirection: "column", alignItems: "center" }}
    >
      <h1>Upload Images</h1>
      <h2>This page allows you to upload an image to the app.</h2>
      <input type="file" />
      <button>Submit</button>
    </div>
  );
};

export default UploadPage;
