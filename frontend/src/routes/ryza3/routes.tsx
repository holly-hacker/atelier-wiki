import { Route } from "react-router-dom";
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

export default function Ryza3Routes() {
  return (
    <Route path="ryza3">
      <Route index element={<Ryza3Index />} />

      <Route path="items" element={<ItemList />} />
      <Route path="items/:id" element={<ItemDetail />} />
      <Route path="item_categories" element={<ItemCategoriesList />} />
      <Route
        path="item_categories/:category"
        element={<ItemCategoryDetail />}
      />
      <Route path="item_kinds" element={<ItemKindsList />} />
      <Route path="item_kinds/:kind" element={<ItemKindDetail />} />
      <Route path="item_use_tags" element={<ItemUseTagsList />} />
      <Route path="item_use_tags/:tag" element={<ItemUseTagDetail />} />

      <Route path="enemies" element={<EnemyList />} />
      <Route path="enemy/:id" element={<EnemyDetail />} />
    </Route>
  );
}
