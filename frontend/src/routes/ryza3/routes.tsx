import { Route } from "react-router-dom";
import Ryza3Index from ".";
import ItemList from "./items/list";
import ItemDetail from "./items/detail";
import ItemCategoriesList from "./item_categories/list";
import ItemCategoryDetail from "./item_categories/detail";
import ItemKindsList from "./item_kinds/list";
import ItemKindDetail from "./item_kinds/detail";

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
    </Route>
  );
}
