import "./App.css";
const SearchPage = () => {
  return (
    <div className="searchpage-background">
      <a href="http://localhost:3004/" target="_blank">
        <button className="home-button">home</button>
      </a>
      <a href="http://localhost:3004/upload" target="_blank">
        <button className="upload-button">upload</button>
      </a>
      <div className="searchpage-container">
        <h1 className="search-page-title">enter your query below</h1>
        <input className="input-bar" />
        <button className="search-button-big">search</button>
      </div>
    </div>
  );
};
export default SearchPage;
