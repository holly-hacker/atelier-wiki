import { Route } from "react-router-dom";
import Ryza3Index from ".";
import ItemList from "./items/list";
import ItemDetail from "./items/detail";

export default function Ryza3Routes() {
  return (
    <Route path="ryza3">
      <Route index element={<Ryza3Index />} />
      <Route path="items" element={<ItemList />} />
      <Route path="item/:id" element={<ItemDetail />} />
    </Route>
  );
}
