import { RouteObject } from "react-router-dom";
import DataLoader from "./data_loader";
import SophieIndex from "./index";
import ItemList from "./items/list";
import ItemDetail from "./items/detail";
import FriendPresentList from "./friends/presents";

export default function getRoutes(): RouteObject[] {
  return [
    {
      path: "sophie",
      element: <DataLoader />,
      children: [
        {
          index: true,
          element: <SophieIndex />,
        },
        {
          path: "items",
          element: <ItemList />,
        },
        {
          path: "items/:id",
          element: <ItemDetail />,
        },
        {
          path: "friends",
          element: <FriendPresentList />,
        },
      ],
    },
  ];
}
