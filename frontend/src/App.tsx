import "./App.css";
import { BrowserRouter, Link, Outlet, Route, Routes } from "react-router-dom";
import Ryza3Routes from "./routes/ryza3/routes";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<IndexPage />} />
          {Ryza3Routes()}
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

function Layout() {
  // we'll add some layout stuff here later
  return (
    <>
      <Link to="/">Home</Link>
      <Outlet />
    </>
  );
}

function IndexPage() {
  return (
    <>
      <h1>Index page</h1>
      <ul>
        <li>
          <Link to="/ryza3">Atelier Ryza 3</Link>
        </li>
      </ul>
    </>
  );
}

export default App;