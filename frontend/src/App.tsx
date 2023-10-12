import "./App.css";
import {
  Link,
  Outlet,
  Route,
  RouterProvider,
  createHashRouter,
  createRoutesFromElements,
} from "react-router-dom";
import Ryza3Routes from "./routes/ryza3/routes";
import DataLoader from "./data_loader";

function App() {
  // TODO: use proper router data instead of `createRoutesFromElements`
  const router = createHashRouter(
    createRoutesFromElements(
      <Route path="/" element={<Layout />}>
        <Route index element={<IndexPage />} />
        {Ryza3Routes()}
      </Route>,
    ),
  );
  return <RouterProvider router={router} />;
}

function Layout() {
  // we'll add some layout stuff here later
  return (
    <>
      <div>
        <Link to="/">Home</Link>
      </div>
      <DataLoader>
        <Outlet />
      </DataLoader>
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
