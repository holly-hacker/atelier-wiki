import { RouteObject } from "react-router-dom";
import Ryza3Index from ".";
import ItemList from "./items/list";
import ItemDetail from "./items/detail";
import ItemCategoriesList from "./item_categories/list";
import ItemCategoryDetail from "./item_categories/detail";
import ItemKindsList from "./item_kinds/list";
import ItemKindDetail from "./item_kinds/detail";
import ItemUseTagsList from "./item_use_tags/list";
import ItemUseTagDetail from "./item_use_tags/detail";
import EnemyList from "./enemies/list";
import EnemyDetail from "./enemies/detail";
import Ryza3Map from "./map/map";

export default function getRoutes(): RouteObject[] {
  return [
    {
      path: "ryza3",
      children: [
        {
          index: true,
          element: <Ryza3Index />,
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
          path: "item_categories",
          element: <ItemCategoriesList />,
        },
        {
          path: "item_categories/:category",
          element: <ItemCategoryDetail />,
        },
        {
          path: "item_kinds",
          element: <ItemKindsList />,
        },
        {
          path: "item_kinds/:kind",
          element: <ItemKindDetail />,
        },
        {
          path: "item_use_tags",
          element: <ItemUseTagsList />,
        },
        {
          path: "item_use_tags/:tag",
          element: <ItemUseTagDetail />,
        },
        {
          path: "enemies",
          element: <EnemyList />,
        },
        {
          path: "enemy/:id",
          element: <EnemyDetail />,
        },
        {
          path: "map",
          element: <Ryza3Map />,
        },
      ],
    },
  ];
}
