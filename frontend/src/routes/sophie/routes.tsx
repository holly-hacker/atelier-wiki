import { RouteObject } from "react-router-dom";

export default function getRoutes(): RouteObject[] {
  return [
    {
      path: "sophie",
      element: <>Index</>,
    },
  ];
}
