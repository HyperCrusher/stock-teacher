/* @refresh reload */
import { render } from "solid-js/web";
import "./styles/app.css";
import Login from "./pages/login.tsx";

const root = document.getElementById("root");

render(() => <Login />, root!);
