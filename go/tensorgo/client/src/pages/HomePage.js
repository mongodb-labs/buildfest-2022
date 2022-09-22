import { Link } from "react-router-dom";
import { useEffect, useState } from "react";
import axios from "axios";
import "./App.css";
const sampleImages = [
  "https://images.unsplash.com/photo-1663698833903-c48466f6ce22?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8Mzd8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663703051230-a50d963f0add?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8NDV8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1662581871625-7dbd3ac1ca18?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDF8MXxhbGx8NDZ8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663705253939-99a27f2c5d53?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8NDh8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663573690125-d326a87a2535?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8Mzh8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663691219171-93494f63b5c9?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8Mzl8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1661956601349-f61c959a8fd4?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDF8MXxhbGx8NDF8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663536101998-36c21a2946dc?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8NDN8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663676609844-07dd7294b3cb?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8NDR8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
  "https://images.unsplash.com/photo-1663524023198-1e808ad5e686?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwxMjA3fDB8MXxhbGx8NDd8fHx8fHwyfHwxNjYzNzcwNjQ3&ixlib=rb-1.2.1&q=80&w=1080",
];
const ImageCard = ({ src }) => {
  return (
    <span style={{ margin: "2rem" }}>
      <img style={{ width: "20%" }} src={src} alt="bjhebhb" />
    </span>
  );
};
const HomePage = () => {

    const BACKEND_HOST = process.env.BACKEND_HOST || "localhost";

    const [imageResults, setImageResults] = useState([]);

    useEffect(() => {
        (async () => {
            try {
                const response = await axios({
                    "method": "GET",
                    "url": `http://${BACKEND_HOST}:12345/find`
                });
                setImageResults(response.data);
            } catch (error) {
                console.error(error);
            }
        })();
    }, [BACKEND_HOST]);

  return (
    <div className="homepage-background">
      <Link to={"/search"}>
        <button className="home-button">search</button>
      </Link>
      <Link to={"/upload"}>
        <button className="home-button">upload</button>
      </Link>
      <h1 className="title">tensorGO: homepage</h1>
      <div className="searchResults">
            {imageResults.map((val) => (
                <ImageCard key={val._id} src={"http://" + BACKEND_HOST + ":12345" + val.url} />
            ))}
        </div>
      {sampleImages.map((val) => (
        <ImageCard key={val} src={val} />
      ))}
    </div>
  );
};
export default HomePage;
