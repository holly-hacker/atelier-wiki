import "./App.css";
import {
  Link,
  Outlet,
  RouterProvider,
  createHashRouter,
} from "react-router-dom";
import getSophieRoutes from "./routes/sophie/routes";
import getRyza3Routes from "./routes/ryza3/routes";
import DataLoader from "./data_loader";

function App() {
  const router = createHashRouter([
    {
      path: "/",
      element: <Layout />,
      children: [
        { index: true, element: <IndexPage /> },
        ...getSophieRoutes(),
        ...getRyza3Routes(),
      ],
    },
  ]);
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
          <Link to="/sophie">Atelier Sophie</Link>
        </li>
        <li>
          <Link to="/ryza3">Atelier Ryza 3</Link>
        </li>
      </ul>
    </>
  );
}

export default App;
