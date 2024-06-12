import React from "react";
import ReactDOM from "react-dom/client";
import App from "./modules/App";
import "./styles/styles.css";

document.body.classList.add("bg-neutral-950");

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
