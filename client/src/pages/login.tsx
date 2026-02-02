import logo from "../assets/logo.png";
import "../styles/login.css";

export default function App() {
  return (
    <div class="container">
      <div class="card">
        <img src={logo} alt="Stock Teacher logo" />

        <div class="logo-text">
          <h1>Stock</h1>
          <h1>Teacher</h1>
        </div>

        <form>
          <input type="text" placeholder="Game Code" />
          <button>Join Game</button>
          <button>Create Game</button>
        </form>
      </div>
    </div>
  );
}
