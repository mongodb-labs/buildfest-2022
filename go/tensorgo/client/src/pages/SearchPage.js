import "./App.css";
import { Link } from "react-router-dom";

const SearchPage = () => {
  return (
    <div className="searchpage-background">
      <Link to={"/"}>
        <button className="home-button">home</button>
      </Link>
      <Link to={"/upload"}>
        <button className="home-button">upload</button>
      </Link>
      <div className="searchpage-container">
        <h1 className="search-page-title">enter your query below</h1>
        <input className="input-bar" />
        <button className="search-button-big">search</button>
      </div>
    </div>
  );
};
export default SearchPage;
