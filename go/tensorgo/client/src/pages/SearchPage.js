import "./App.css";
import { Link } from "react-router-dom";
import { useState } from "react";
import axios from "axios";

const ImageCard = ({ src }) => {
    return (
        <span style={{ margin: "2rem" }}>
        <img style={{ width: "20%" }} src={src} alt="bjhebhb" />
        </span>
    );
};

const SearchPage = () => {

    const BACKEND_HOST = process.env.BACKEND_HOST || "localhost";

    const [searchLabel, setSearchLabel] = useState("");
    const [searchThreshold, setSearchThreshold] = useState(0);
    const [imageResults, setImageResults] = useState([]);

    const handleSubmit = async (event) => {
        event.preventDefault();
        let queryParams = {
            threshold: searchThreshold
        };
        if(searchLabel !== "") {
            queryParams.label = searchLabel;
        }
        try {
            const response = await axios({
                "method": "GET",
                "url": `http://${BACKEND_HOST}:12345/find`,
                "params": queryParams
            });
            setImageResults(response.data);
        } catch (error) {
            console.error(error);
        }
    }

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
                <input 
                    value={searchLabel} 
                    onChange={e => setSearchLabel(e.target.value)}
                    name="searchLabel" 
                    className="input-bar" 
                    placeholder="Search Label..." 
                />
                <input 
                    value={searchThreshold} 
                    onChange={e => setSearchThreshold(e.target.value)}
                    name="searchThreshold" 
                    className="input-bar" 
                    placeholder="Threshold [0-1]..." 
                />
                <button className="search-button-big" onClick={handleSubmit}>search</button>
            </div>
            <div className="searchResults">
                {imageResults.map((val) => (
                    <ImageCard key={val._id} src={"http://" + BACKEND_HOST + ":12345" + val.url} />
                ))}
            </div>
        </div>
    );
};
export default SearchPage;
